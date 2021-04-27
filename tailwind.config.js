module.exports = {
    mod: 'jit',
    purge: [
        './src/**/*.rs',
    ],

    purge: {
        enabled: true,
        mode: 'all',
        // source_code represents the rust (yew?) source code root
        content: ["./src/**/*.rs", "./static/index.html", "./css/index.css"]
    },
    theme: {},
    variants: {},
    plugins: [],
}