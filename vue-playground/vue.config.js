const { defineConfig } = require("@vue/cli-service");
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
          use: "file-loader",
        },
      ],
    },
  },
});
