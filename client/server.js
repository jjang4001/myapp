let webpackFile = process.env.NODE_ENV === 'development' ? './webpack/webpack.dev.js' : './webpack/webpack.prod.js';

const path = require('path'),
    express = require('express'),
    webpack = require('webpack'),
    webpackConfig = require(webpackFile),
    app = express(),
    port = process.env.PORT || 3000;

// TODO: Once express adds support for wasm, update express version
express.static.mime.types['wasm'] = 'application/wasm'

app.listen(port, () => { console.log(`App is listening on port ${port}`) });
app.use('/', express.static(path.join(__dirname, 'dist/js')));

app.get('/', (req, res) => {
    res.sendFile(path.resolve(__dirname, 'dist/js', 'index.html'));
});

let compiler = webpack(webpackConfig);

if (process.env.NODE_ENV === 'development') {
    app.use(require('webpack-dev-middleware')(compiler, {
        noInfo: true, publicPath: webpackConfig.output.publicPath, stats: { colors: true }
    }));

    app.use(require('webpack-hot-middleware')(compiler));
}
