use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    sync::Arc,
};

use eyre::{OptionExt, WrapErr, eyre};
use fs4::fs_std::FileExt;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::AppHandle;
use tokio::task::JoinSet;
use tracing::{Instrument, instrument};

use crate::{
    config::FileExistAction,
    downloader::{
        download_chunk_task::DownloadChunkTask, download_progress::DownloadProgress,
        download_task::DownloadTask, media_chunk::MediaChunk,
    },
    extensions::{AppHandleExt, EyreReportToMessage},
    types::{
        bangumi_media_url::BangumiMediaUrl, cheese_media_url::CheeseMediaUrl,
        codec_type::CodecType, normal_media_url::NormalMediaUrl, video_quality::VideoQuality,
    },
    utils,
};

const CHUNK_SIZE: u64 = 2 * 1024 * 1024; // 2MB

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(default)]
pub struct VideoTask {
    pub selected: bool,
    pub url: String,
    pub video_quality: VideoQuality,
    pub codec_type: CodecType,
    pub content_length: u64,
    pub chunks: Vec<MediaChunk>,
    pub completed: bool,
    pub skipped: bool,
}

impl VideoTask {
    #[instrument(level = "error", skip_all)]
    pub async fn prepare_normal(
        &mut self,
        app: &AppHandle,
        media_url: &NormalMediaUrl,
    ) -> eyre::Result<()> {
        let mut join_set = JoinSet::new();

        for media in &media_url.dash.video {
            let app = app.clone();
            let id = media.id;
            let codecid = media.codecid;

            let mut urls = Vec::new();
            urls.extend_from_slice(&media.backup_url);
            urls.push(media.base_url.clone());

            let get_url_with_content_length_task = async move {
                let bili_client = app.get_bili_client();
                let url_with_content_length = bili_client.get_url_with_content_length(urls).await;
                MediaForPrepare {
                    id,
                    url_with_content_length,
                    codecid,
                }
            };

            join_set.spawn(get_url_with_content_length_task.in_current_span());
        }

        for durl in &media_url.durl {
            let app = app.clone();
            let id = media_url.quality;
            let codecid = media_url.video_codecid;

            let mut urls = Vec::new();
            urls.extend_from_slice(&durl.backup_url);
            urls.push(durl.url.clone());

            let get_url_with_content_length_task = async move {
                let bili_client = app.get_bili_client();
                let url_with_content_length = bili_client.get_url_with_content_length(urls).await;
                MediaForPrepare {
                    id,
                    url_with_content_length,
                    codecid,
                }
            };

            join_set.spawn(get_url_with_content_length_task.in_current_span());
        }

        let mut medias: Vec<MediaForPrepare> = Vec::new();

        while let Some(join_result) = join_set.join_next().await {
            let Ok(media) = join_result else {
                continue;
            };

            if !media.url_with_content_length.is_empty() {
                medias.push(media);
            }
        }

        self.prepare(app, &medias)?;

        Ok(())
    }

    #[instrument(level = "error", skip_all)]
    pub async fn prepare_bangumi(
        &mut self,
        app: &AppHandle,
        media_url: &BangumiMediaUrl,
    ) -> eyre::Result<()> {
        let mut medias: Vec<MediaForPrepare> = Vec::new();

        let mut join_set = JoinSet::new();

        if let Some(dash) = &media_url.dash {
            for media in &dash.video {
                let app = app.clone();
                let id = media.id;
                let codecid = media.codecid;

                let mut urls = Vec::new();
                urls.extend_from_slice(&media.backup_url);
                urls.push(media.base_url.clone());

                let get_url_with_content_length_task = async move {
                    let bili_client = app.get_bili_client();
                    let url_with_content_length =
                        bili_client.get_url_with_content_length(urls).await;
                    MediaForPrepare {
                        id,
                        url_with_content_length,
                        codecid,
                    }
                };

                join_set.spawn(get_url_with_content_length_task.in_current_span());
            }
        }

        for durl in &media_url.durls {
            for media in &durl.durl {
                let app = app.clone();
                let id = durl.quality;
                let codecid = media_url.video_codecid;

                let mut urls = Vec::new();
                urls.extend_from_slice(&media.backup_url);
                urls.push(media.url.clone());

                let get_url_with_content_length_task = async move {
                    let bili_client = app.get_bili_client();
                    let url_with_content_length =
                        bili_client.get_url_with_content_length(urls).await;
                    MediaForPrepare {
                        id,
                        url_with_content_length,
                        codecid,
                    }
                };

                join_set.spawn(get_url_with_content_length_task.in_current_span());
            }
        }

        while let Some(join_result) = join_set.join_next().await {
            let Ok(media) = join_result else {
                continue;
            };

            if !media.url_with_content_length.is_empty() {
                medias.push(media);
            }
        }

        self.prepare(app, &medias)?;

        Ok(())
    }

    #[instrument(level = "error", skip_all)]
    pub async fn prepare_cheese(
        &mut self,
        app: &AppHandle,
        media_url: &CheeseMediaUrl,
    ) -> eyre::Result<()> {
        let mut medias: Vec<MediaForPrepare> = Vec::new();

        let mut join_set = JoinSet::new();

        if let Some(dash) = &media_url.dash {
            for media in &dash.video {
                let app = app.clone();
                let id = media.id;
                let codecid = media.codecid;

                let mut urls = Vec::new();
                urls.extend_from_slice(&media.backup_url);
                urls.push(media.base_url.clone());

                let get_url_with_content_length_task = async move {
                    let bili_client = app.get_bili_client();
                    let url_with_content_length =
                        bili_client.get_url_with_content_length(urls).await;
                    MediaForPrepare {
                        id,
                        url_with_content_length,
                        codecid,
                    }
                };

                join_set.spawn(get_url_with_content_length_task.in_current_span());
            }
        }

        for durl in &media_url.durls {
            for media in &durl.durl {
                let app = app.clone();
                let id = durl.quality;
                let codecid = media_url.video_codecid;

                let mut urls = Vec::new();
                urls.extend_from_slice(&media.backup_url);
                urls.push(media.url.clone());

                let get_url_with_content_length_task = async move {
                    let bili_client = app.get_bili_client();
                    let url_with_content_length =
                        bili_client.get_url_with_content_length(urls).await;
                    MediaForPrepare {
                        id,
                        url_with_content_length,
                        codecid,
                    }
                };

                join_set.spawn(get_url_with_content_length_task.in_current_span());
            }
        }

        while let Some(join_result) = join_set.join_next().await {
            let Ok(media) = join_result else {
                continue;
            };

            if !media.url_with_content_length.is_empty() {
                medias.push(media);
            }
        }

        self.prepare(app, &medias)?;

        Ok(())
    }

    #[instrument(level = "error", skip_all)]
    fn prepare(&mut self, app: &AppHandle, medias: &[MediaForPrepare]) -> eyre::Result<()> {
        if medias.is_empty() {
            return Err(eyre!("获取视频地址失败，medias为空"));
        }

        let video_quality_is_unknown = self.video_quality == VideoQuality::Unknown;
        let codec_type_is_unknown = self.codec_type == CodecType::Unknown;

        if video_quality_is_unknown != codec_type_is_unknown {
            return Err(eyre!(
                "`video_quality`和`codec_type`必须同时为`Unknown`或同时不为`Unknown`"
            ));
        }

        // 如果`video_quality`和`codec_type`同时为`Unknown`，则更倾向于使用优先级选择
        let prefer_select_by_priority = video_quality_is_unknown;

        let selected_media = if prefer_select_by_priority {
            select_media_by_priority(app, medias)
        } else {
            select_exact_match_media(self, medias).or_else(|| select_media_by_priority(app, medias))
        };

        let media = selected_media.ok_or_eyre("获取视频地址失败，medias为空")?;

        self.video_quality = media.id.into();
        self.codec_type = media.codecid.into();

        let (url, content_length) = media
            .url_with_content_length
            .iter()
            .find(|(url, _)| url.starts_with("https://upos-"))
            .unwrap_or(&media.url_with_content_length[0])
            .clone();

        self.url = url;

        if self.content_length != content_length {
            let chunk_count = content_length.div_ceil(CHUNK_SIZE);

            #[allow(clippy::cast_possible_truncation)]
            let mut chunks = Vec::with_capacity(chunk_count as usize);
            for i in 0..chunk_count {
                let start = i * CHUNK_SIZE;
                let end = std::cmp::min(start + CHUNK_SIZE, content_length) - 1;
                chunks.push(MediaChunk {
                    start,
                    end,
                    completed: false,
                });
            }

            self.content_length = content_length;
            self.chunks = chunks;
        }

        Ok(())
    }

    pub fn mark_uncompleted(&mut self) {
        self.completed = false;
        self.chunks.iter_mut().for_each(|chunk| {
            chunk.completed = false;
        });
        self.skipped = false;
    }

    pub fn is_completed(&self) -> bool {
        !self.selected || self.completed
    }

    #[allow(clippy::too_many_lines)]
    #[instrument(level = "error", skip_all)]
    pub async fn process(
        &self,
        download_task: &Arc<DownloadTask>,
        progress: &DownloadProgress,
    ) -> eyre::Result<()> {
        let (episode_dir, filename) = (&progress.episode_dir, &progress.filename);
        let video_task = download_task.progress.read().video_task.clone();

        let mp4_path = episode_dir.join(format!("{filename}.mp4"));
        let file_exist_action = download_task.app.get_config().read().file_exist_action;
        if file_exist_action == FileExistAction::Skip && mp4_path.exists() {
            tracing::debug!("视频文件已存在，跳过下载");
            download_task.update_progress(|p| {
                p.video_task.skipped = true;
                p.video_task.completed = true;
            });
            return Ok(());
        }

        let temp_file_path = episode_dir.join(format!(
            "{filename}.mp4.com.lanyeeee.bilibili-video-downloader"
        ));

        let should_reuse_temp_file = temp_file_path
            .metadata()
            .map(|m| m.len() == video_task.content_length)
            .unwrap_or(false);

        let file = if should_reuse_temp_file {
            // 如果临时文件可以重用，则直接打开它
            OpenOptions::new()
                .read(true)
                .write(true)
                .open(&temp_file_path)?
        } else {
            // 如果临时文件不能重用，则创建个新的
            let file = File::create(&temp_file_path)?;
            file.allocate(video_task.content_length)?;
            file
        };
        let file = Arc::new(Mutex::new(file));

        let chunk_count = video_task.chunks.len();

        let mut join_set = JoinSet::new();
        for (chunk_index, chunk) in video_task.chunks.iter().enumerate() {
            if chunk.completed {
                continue;
            }

            let (start, end) = (chunk.start, chunk.end);

            let download_chunk_task = DownloadChunkTask {
                download_task: download_task.clone(),
                start,
                end,
                url: video_task.url.clone(),
                file: file.clone(),
                chunk_index,
            };

            let chunk_order = chunk_index + 1;
            let chunk_task = async move {
                download_chunk_task.process().await.wrap_err(format!(
                    "分片`{chunk_order}/{chunk_count}`下载失败({start}-{end})"
                ))
            };
            join_set.spawn(chunk_task.in_current_span());
        }

        while let Some(join_result) = join_set.join_next().await {
            let Ok(download_video_result) = join_result else {
                continue;
            };

            match download_video_result {
                Ok(i) => download_task.update_progress(|p| p.video_task.chunks[i].completed = true),
                Err(err) => {
                    let err_title = "视频的一个分片下载失败";
                    let message = err.to_message();
                    tracing::error!(err_title, message);
                }
            }
        }
        // 检查视频是否已下载完成
        let download_completed = download_task
            .progress
            .read()
            .video_task
            .chunks
            .iter()
            .all(|chunk| chunk.completed);
        if !download_completed {
            return Err(eyre!(
                "视频文件`{}`有分片未下载完成，[继续]可以跳过已下载分片断点续传",
                temp_file_path.display()
            ));
        }

        let is_video_file_complete = utils::is_mp4_complete(&temp_file_path).wrap_err(format!(
            "检查视频文件`{}`是否完整失败",
            temp_file_path.display()
        ))?;

        if !is_video_file_complete {
            download_task.update_progress(|p| p.video_task.mark_uncompleted());
            return Err(eyre!(
                "视频文件`{}`不完整，[继续]会重新下载所有分片",
                temp_file_path.display()
            ));
        }

        // 重命名临时文件
        if mp4_path.exists() {
            std::fs::remove_file(&mp4_path)
                .wrap_err(format!("删除已存在的视频文件`{}`失败", mp4_path.display()))?;
        }
        std::fs::rename(&temp_file_path, &mp4_path).wrap_err(format!(
            "将临时文件`{}`重命名为`{}`失败",
            temp_file_path.display(),
            mp4_path.display()
        ))?;

        download_task.update_progress(|p| p.video_task.completed = true);

        Ok(())
    }
}

#[derive(Clone)]
struct MediaForPrepare {
    pub id: i64,
    pub url_with_content_length: Vec<(String, u64)>,
    pub codecid: i64,
}

fn select_exact_match_media(
    video_task: &VideoTask,
    medias: &[MediaForPrepare],
) -> Option<MediaForPrepare> {
    let media = medias.iter().find(|media| {
        let quality: VideoQuality = media.id.into();
        let codec_type: CodecType = media.codecid.into();
        quality == video_task.video_quality && codec_type == video_task.codec_type
    });

    media.cloned()
}

fn select_media_by_priority(
    app: &AppHandle,
    medias: &[MediaForPrepare],
) -> Option<MediaForPrepare> {
    let (video_quality_priority, codec_type_priority) = {
        let config = app.get_config().inner().read();
        (
            config.video_quality_priority.clone(),
            config.codec_type_priority.clone(),
        )
    };

    // 构建索引表，这是为了在排序时能以 O(1) 查找到优先级，索引越小优先级越高
    let video_priority_map: HashMap<&VideoQuality, usize> = video_quality_priority
        .iter()
        .enumerate()
        .map(|(index, quality)| (quality, index))
        .collect();
    let codec_priority_map: HashMap<&CodecType, usize> = codec_type_priority
        .iter()
        .enumerate()
        .map(|(index, codec_type)| (codec_type, index))
        .collect();

    let media = medias.iter().min_by_key(|media| {
        let quality: VideoQuality = media.id.into();
        let quality_index = video_priority_map.get(&quality).unwrap_or(&usize::MAX);

        let codec_type: CodecType = media.codecid.into();
        let codec_index = codec_priority_map.get(&codec_type).unwrap_or(&usize::MAX);
        // Rust 的元组比较机制是从左到右依次比较
        // 先比较quality_index(主排序键)
        // 如果quality_index相同，则比较codec_index(次排序键)
        (quality_index, codec_index)
    });

    media.cloned()
}
