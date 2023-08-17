const { defineConfig } = require("@vue/cli-service");
const path = require("path");
module.exports = defineConfig({
  transpileDependencies: true,
  lintOnSave: false,
  configureWebpack: {
    experiments: {
      asyncWebAssembly: true,
    },
    module: {
      rules: [
        {
          test: /\.png/,
          use: {
            loader: "url-loader",
            options: {
              limit: true,
            },
          },
        },
      ],
    },
  },
  chainWebpack: (config) => {
    const imageRule = config.module.rule("images");
    imageRule.delete("type");
  },
});
