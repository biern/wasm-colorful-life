"use strict";
exports.__esModule = true;
var react_dom_1 = require("react-dom");
var react_1 = require("react");
var wasm = Promise.resolve().then(function () { return require("../pkg/index.js"); }).then(function (mod) { return mod; });
wasm["catch"](console.error).then(function (mod) {
    var Game = mod.Game;
    var game = Game["new"](100);
    var initial = game.get_state();
    console.log("initial", initial);
    setInterval(function () {
        var events = game.tick();
        console.log("events", events);
    }, 1000);
});
exports.App = function () {
    return <div />;
};
react_dom_1["default"].render(react_1["default"].createElement(exports.App), document.getElementById("root"));
