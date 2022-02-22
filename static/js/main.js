#![feature(string_remove_matches)]

const leaflet = window.leaflet;

const url = "/search/query";
const myForm = document.getElementById("search_form");

const max_lat = 49.371643;
const min_lat = 25.827089;
const max_long = -66.927119;
const min_long = -124.639440;

myForm.addEventListener('submit', function (e) {
    e.preventDefault();

    const formData = new FormData(this);

    fetch(url, {
        method: 'post',
        body: formData,
    }).then(function (response) {
        return response.json();
    }).then(function (results) {
        map.off();
        map.remove();
        updatePageInfo(results);
        console.log("returned text", results);
    }).catch(function (error) {
        console.log(error);
    })
})

function makeMap(positions, continental_us) {
    var sw = [min_lat, min_long];
    var ne = [max_lat, max_long];

    var filtered_positions = [];
    if (positions.length > 0) {
        if (continental_us) {
            for (const job of positions) {
                if (min_lat < job.latitude && job.latitude < max_lat && min_long < job.longitude < max_long) {
                    filtered_positions.push(job);
                }
            }
        } else {
            filtered_positions = positions;
        }

        var n = -100;
        var s = 100;
        var e = -180;
        var w = 180;
        for (const job of positions) {
            if (e < job.latitude) {
                e = job.latitude;
            } else if (job.latitude < w) {
                w = job.latitude;
            }
            if (n < job.longitude) {
                n = job.longitude;
            } else if (job.longitude < s) {
                s = job.longitude;
            }
        }

        sw = [parseFloat(w), parseFloat(s)];
        ne = [parseFloat(e), parseFloat(n)];
    }

    var start_coords = [(sw[0] + ne[0]) / 2, (sw[1] + ne[1]) / 2];
    var map = L.map('mapid').fitBounds([sw, ne]);
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png?{foo}', {foo: 'bar', attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}).addTo(map);

    console.log('loaded');
}

function updatePageInfo(pageInfo) {
    document.getElementById("page").textContent = pageInfo.current_page;
    document.getElementById("number-of-pages").textContent = pageInfo.number_of_pages;

    document.getElementById("total_search_results").textContent = pageInfo.total_search_results;
    document.getElementById("total_returned_jobs").textContent = pageInfo.positions.length;
    document.getElementById("total_returned_locations").textContent = pageInfo.total_returned_locations;

    makeMap(pageInfo.positions, pageInfo.continental_us);
}

makeMap([], false);