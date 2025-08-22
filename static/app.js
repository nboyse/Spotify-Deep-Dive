let chart;
let activeIndex = null;

function getBarColors(labels) {
    const palette = [
        '#FF6384', '#36A2EB', '#FFCE56', '#4BC0C0',
        '#9966FF', '#FF9F40', '#00C49F', '#E91E63',
        '#2196F3', '#FFC107'
    ];
    return labels.map((_, i) => {
        if (i === activeIndex) {
            return palette[i % palette.length];
        } else {
            return palette[i % palette.length] + '99';
        }
    });
}

async function loadData() {
    const resp = await fetch('/genres');
    const data = await resp.json();

    const labels = data.map(item => item.genre);
    const counts = data.map(item => item.count);

    const ctx = document.getElementById('chart').getContext('2d');
    chart = new Chart(ctx, {
        type: 'bar',
        data: {
            labels,
            datasets: [{
                label: 'Top Genres',
                data: counts,
                backgroundColor: getBarColors(labels),
            }]
        },
        options: {
            responsive: true,
            plugins: {
            legend: {
                labels: { color: 'white' }
            },
            tooltip: {
                bodyColor: 'white',
                titleColor: 'white'
            }
        },
        scales: {
            x: {
                ticks: { color: 'white' },
                grid: { color: '#333' }
            },
            y: {
                beginAtZero: true,
                ticks: { color: 'white' },
                grid: { color: '#333' }
            }
        },
            onClick: (evt, elements) => {
                if (elements.length > 0) {
                    const idx = elements[0].index;
                    activeIndex = idx; // store which bar was clicked
                    chart.data.datasets[0].backgroundColor = getBarColors(chart.data.labels);
                    chart.update();

                    const genre = chart.data.labels[idx];
                    loadGenreDetails(genre);
                }
            }
        }
    });
}

async function loadGenreDetails(genre) {
    const resp = await fetch(`/genres/${encodeURIComponent(genre)}`);
    const details = await resp.json();

    const container = document.getElementById('details');
    container.innerHTML = `
        <h3>Artists & Tracks in ${genre}</h3>
        <table>
            <thead>
                <tr><th>Artist</th><th>Tracks</th></tr>
            </thead>
            <tbody>
                ${details.map(d => `
                    <tr>
                        <td>${d.artist}</td>
                        <td>${d.tracks.join(', ')}</td>
                    </tr>
                `).join('')}
            </tbody>
        </table>
    `;
    container.style.display = 'block';
}

loadData();
