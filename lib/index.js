var addon = require('../native');

console.log(addon.hello());

setTimeout(function() {
    console.log("Called in setTimeout");
}, 5);