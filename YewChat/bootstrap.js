import('./pkg/yewchat.js').then((module) => {
    return module.default().then(() => module.run_app());
});
