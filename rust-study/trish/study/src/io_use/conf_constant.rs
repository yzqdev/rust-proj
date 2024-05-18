
pub const UNBUILD_CONF:&str = r#"import { defineBuildConfig } from "UNBUILD";

pub default defineBuildConfig({
  entries: ["./src/index"],
  // entries: ["src/"],
  rollup: {
    emitCJS: true,
    inlineDependencies: false,
  },
  clean: true,
  declaration: true,
});
"#;

pub const TSCONF:&str  = r#"
{
  "compilerOptions": {
    "sourceMap": true,
    "outDir": "dist",
    "target": "esnext",
    "module": "esnext",
    "moduleResolution": "Bundler",
    "strict": false,
    "lib": ["esnext"],
    "experimentalDecorators": true,
    "emitDecoratorMetadata": true,
    "esModuleInterop": true,
    //"verbatimModuleSyntax": true,
    "resolveJsonModule": true
  }
}
"#;
pub const TSUP_CONF:&str  = r#"import { defineConfig } from "tsup";

pub default defineConfig({
  entry: ["src/"],
  splitting: false,
  sourcemap: false,
  minify: false,
  format: ["esm", "cjs"],
  dts: true,
  clean: true,
});
";
pub const unoConf:&str  = "import {
  defineConfig,
  presetAttributify,
  presetIcons,
  presetUno,
} from "unocss";

pub default defineConfig({
  rules: [["custom-rule", { color: "red" }]],
  shortcuts: {
    "custom-shortcut": "text-lg text-orange hover:text-teal",
  },
  presets: [
    presetUno(),
    presetAttributify(),
    presetIcons({
      scale: 1.2,
      cdn: "https://esm.sh/",
    }),
  ],
});
"#;
pub const RSBUILD_CONF:&str  = r#"
import { defineConfig } from "@rsbuild/core";
import { pluginBabel } from "@rsbuild/plugin-babel";
import { pluginSolid } from "@rsbuild/plugin-solid";
import { UserScriptMetaDataPlugin } from "./userScriptPlugin";
import { rspack } from "@rsbuild/core";
pub default defineConfig({
  output: {
    distPath: { js: "." },
    injectStyles: true,
    filenameHash: false,
    cleanDistPath: true,
    filename: {
      js: "solid.user.js",
    },
    minify: true,
  },
  tools: {
    htmlPlugin: false,
    bundlerChain(chain, utils) {
      chain.devtool(false);
      console.log(new Date());
      console.log(utils);
    },
    rspack: {
      plugins: [
        new rspack.SwcJsMinimizerRspackPlugin({
          format: {
            comments: false,
          },
        }),
        new UserScriptMetaDataPlugin({
          metadata: {
            name: "solid",
            require: [
              "https://registry.npmmirror.com/react/18.3.1/files/umd/react.production.min.js",
              "https://registry.npmmirror.com/react-dom/18.3.1/files/umd/react-dom.production.min.js",
              "https://registry.npmmirror.com/dayjs/1.11.11/files/dayjs.min.js",
              "https://registry.npmmirror.com/antd/5.17.0/files/dist/antd.min.js",
            ],
          },
          test: /\.user\.js$/,
        }),
      ],
    },
  },
  plugins: [
    pluginBabel({
      include: /\.(?:jsx|tsx)$/,
      exclude: /[\\/]node_modules[\\/]/,
    }),
    pluginSolid(),
  ],
  performance: {
    chunkSplit: { strategy: "all-in-one" },
  },
});
"#;
pub const GULP_CONF:&str  = r#"
import * as gulp from "gulp";
import * as gulpExe from "gulp-execa";

function buildFun() {
  gulpExe.exec("yarn build");
}
function watchConf() {
  gulp.watch([ "rsbuild.config.ts"], (cb) => {
    console.log(new Date());
    gulpExe.exec("yarn build");
    cb();
  });
}

pub const dev = gulp.parallel(buildFun, watchConf);
"#;
