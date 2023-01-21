const path = require('path')
const { VueLoaderPlugin } = require('vue-loader')

module.exports = {
    entry: './App.vue',
    output: {
        path: path.join(__dirname, './dist'),
        filename: 'app.vue.js',
        libraryTarget: 'umd', 
        library: 'AppVue',
        globalObject: 'this'
    },
    module: {
        rules:[
            {
                test: /\.vue$/,
                loader: 'vue-loader',
                options: {}
            }
        ]
    },
    plugins: [
        new VueLoaderPlugin()
    ]
}
