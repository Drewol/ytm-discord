// ==UserScript==
// @name         YTM2JSON
// @version      0.1
// @description  Takes currently playing YTM and forwards it through a websocket.
// @author       Drewol
// @match        https://music.youtube.com/*
// @grant        none
// ==/UserScript==

(() => {
    const serverAddr = 'ws://127.0.0.1:8975';
    let ws = null;
    const connectWs = () => {
        ws = new WebSocket(serverAddr);
        ws.onmessage = e => console.log('message received');
        ws.onclose = () => setTimeout(connectWs, 5000);
    }
    connectWs();

    var prevSongJson = null;
    setInterval(() => {
        const song = {playing: false};
        const songQueueItem = document.querySelector('#queue ytmusic-player-queue-item[selected]');
        if (songQueueItem) {
            song.playing = songQueueItem.getAttribute('play-button-state') == 'playing';
            song.title = songQueueItem.querySelector('.song-info .song-title')?.getAttribute('title');
            song.artist = songQueueItem.querySelector('.song-info .byline')?.getAttribute('title');
        }
        const songJson = JSON.stringify(song);
        if (songJson !== prevSongJson) {
            try {
                console.log('sending', songJson);
                ws.send(songJson);
                prevSongJson = songJson;
            } catch (e) {
                console.error('failed to send song update', e);
            }
        }
    }, 100);
})();
