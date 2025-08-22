async function loadRecommendation() {
    const resp = await fetch("/recommend_json");
    const rec = await resp.json();

    const img = document.getElementById("rec-img");
    const trackEl = document.getElementById("rec-track");
    const artistEl = document.getElementById("rec-artist");
    const linkEl = document.getElementById("rec-link");

    trackEl.textContent = rec.track;
    artistEl.textContent = rec.artist;
    linkEl.href = rec.spotify_url;

    if (rec.album_image) {
        img.src = rec.album_image;
        img.style.display = "block";
    } else {
        img.style.display = "none";
    }
}

document.getElementById("new-recommend").addEventListener("click", () => {
    loadRecommendation();
});

loadRecommendation();
