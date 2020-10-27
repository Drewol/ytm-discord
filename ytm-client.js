// ==UserScript==
// @name         YTM2JSON
// @namespace    http://tampermonkey.net/
// @version      0.1
// @description  Takes currently playing YTM and forwards it through a websocket.
// @author       Drewol
// @match        https://music.youtube.com/*
// @grant        none
// ==/UserScript==

let ws = null;


function start(websocketServerLocation) {
    ws = new WebSocket(websocketServerLocation);
    ws.onmessage = function (evt) { console.log('message received'); };
    ws.onclose = function () {
        // Try to reconnect in 5 seconds
        setTimeout(function () { start(websocketServerLocation) }, 5000);
    };
}

(function () {
    start("ws://127.0.0.1:8975");

    var prevSong = null;
    'use strict';
    var pollTimer = setInterval(function () {
        try {
            var infoBar = document.getElementsByClassName("content-info-wrapper style-scope ytmusic-player-bar").item(0);
            if (!infoBar)
                return;
            var songTitle = infoBar.getElementsByClassName("title").item(0).getAttribute("title");
            var songSubtitle = infoBar.getElementsByClassName("subtitle").item(0).firstElementChild;
            var songMeta = [];
            for (var i = 0; i < songSubtitle.children.length; i += 2) {
                songMeta.push(songSubtitle.children[i].textContent);
            }

            var controlButton = document.getElementById("play-pause-button");
            var playing = false;
            if (controlButton) {
                playing = controlButton.title === "Pause" || controlButton.hidden;
            }

            var songArtists = [];
            for (i = 0; i < songMeta.length - 2; i++) {
                songArtists.push(songMeta[i]);
            }
            var song = {
                title: songTitle,
                artists: songArtists,
                album: songMeta[songMeta.length - 2],
                year: parseInt(songMeta[songMeta.length - 1]),
                playing: playing
            };
            if (song) {
                var songText = JSON.stringify(song);
                if (songText !== prevSong) {
                    ws.send(songText);
                    prevSong = songText;
                }
            }
        }
        catch (e) { }
    }, 100);
})();