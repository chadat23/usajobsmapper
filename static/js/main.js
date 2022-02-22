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
        updatePageInfo(results);
    }).catch(function (error) {
        console.log(error);
    })
})

function makeMap(positions, continental_us) {

    var sw = [min_lat, min_long];
    var ne = [max_lat, max_long];

    var locations = {};
    var lat = [];
    var long = [];
    if (positions.length > 0) {
        for (const position of positions) {
            for (const location of position.locations) {
                if (continental_us) {
                    if (min_lat < location.latitude && location.latitude < max_lat && min_long < location.longitude && location.longitude< max_long) {
                        lat.push(parseFloat(location.latitude));
                        long.push(parseFloat(location.longitude));
                        if (location in locations) {
                            locations[location.name][0].push(position);
                        } else {
                            locations[location.name] = [[position, ], [lat[lat.length - 1], long[long.length - 1]]];
                        }
                    }
                } else {
                        lat.push(parseFloat(location.latitude));
                        long.push(parseFloat(location.longitude));
                    if (location in locations) {
                        locations[location.name][0].push(position);
                    } else {
                        locations[location.name] = [[position, ], [lat[lat.length - 1], long[long.length - 1]]];
                    }
                }
            }
        }

        sw = [Math.min(...lat), Math.min(...long)];
        ne = [Math.max(...lat), Math.max(...long)];
    }

    console.log(sw, ne);

    var container = L.DomUtil.get('map'); 
    if(container != null){ 
        container._leaflet_id = null; 
    }

    var map = L.map('map').fitBounds([sw, ne]);
    // var map = L.map('map', {maxBounds: [sw, ne]}).fitBounds([sw, ne]);
    // var map = L.map('map', {
    //     center: [51.505, -0.09],
    //     zoom: 10
    // });
    // map.fitBounds([sw, ne]);
    
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png?{foo}', 
        {foo: 'bar', attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}).addTo(map);

    var location_labels = makeLabels(locations);

    var markers = []
    for (const location of location_labels) {
        console.log(location)
        markers.push(L.marker(location["lat_long"]).addTo(map));
    }
}

function updatePageInfo(pageInfo) {
    document.getElementById("page").textContent = pageInfo.current_page;
    document.getElementById("number-of-pages").textContent = pageInfo.number_of_pages;

    document.getElementById("total_search_results").textContent = pageInfo.total_search_results;
    document.getElementById("total_returned_jobs").textContent = pageInfo.positions.length;
    document.getElementById("total_returned_locations").textContent = pageInfo.total_returned_locations;

    makeMap(pageInfo.positions, pageInfo.continental_us);
}

function makeLabels(locations) {
    var location_labels = [];

    for (const [location_name, info] of Object.entries(locations)) {
        toolTipText = "";
        for (const position of info[0]) {
            toolTipText += toolTip(position, location_name);
        }
        console.log(location_labels, info);
        location_labels.push({
            "tooltip": toolTipText,
            "lat_long": info[1],
        });
    }

    return location_labels
}

function toolTip(job, location) {
    return job.title + "<br>" + location + "<br>"
}

makeMap([], false);