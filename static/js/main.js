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
        // reset();
        updatePageInfo(results);
        // console.log("returned text", results);
    }).catch(function (error) {
        console.log(error);
    })
})

// function reset() {
//     document.getElementById("map_holder").innerHTML = '<div class="row map" id="map" style="height: 1000px;">' + 
//     '<div class="button-group">Page:<a href="" class="btn btn-sm btn-nav"><<</a><a href="" class="btn btn-sm btn-nav"><</a>' + 
//     '<span id="current_page"></span><a href="" class="btn btn-sm btn-nav">></a><a href="" class="btn btn-sm btn-nav">>>' + 
//     '</a>{{ content.number_of_pages }}</div></div>'
// }

function makeMap(jobs, positions, continental_us) {

    var sw = [min_lat, min_long];
    var ne = [max_lat, max_long];

    var locations = [];
    if (positions.length > 0) {
        if (continental_us) {
            for (const position of positions) {
                for (const location of position.locations) {
                    if (min_lat < location.latitude && location.latitude < max_lat && min_long < location.longitude && location.longitude< max_long) {
                        locations.push(location);
                    }
                }
            }
        } else {
            for (const position of positions) {
                for (const location of position.locations) {
                    locations.push(location);
                }
            }
        }

        var lat = [];
        var long = [];
        for (const location of locations) {
            lat.push(parseFloat(location.latitude));
            long.push(parseFloat(location.longitude));
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

    // for (const job in jobs) {
    //     var marker = L.marker(job[0], job[1]);

    //     markers.addLayer(marker);
    // }
}

function updatePageInfo(pageInfo) {
    document.getElementById("page").textContent = pageInfo.current_page;
    document.getElementById("number-of-pages").textContent = pageInfo.number_of_pages;

    document.getElementById("total_search_results").textContent = pageInfo.total_search_results;
    document.getElementById("total_returned_jobs").textContent = pageInfo.positions.length;
    document.getElementById("total_returned_locations").textContent = pageInfo.total_returned_locations;

    let jobs = makeLabels(pageInfo.positions);

    makeMap(jobs, pageInfo.positions, pageInfo.continental_us);
}

function makeLabels(positions) {
    var locations = {};

    for (const job in positions) {
        for (const location in job.locations) {
            if ((location.name in locations) == false) {
                locations[location.name] = [toolTip(job, location), [parseFloat(location.latitude), parseFloat(location.longitude)]];
            }
        }
    }

    locations
}

function toolTip(job, location) {
    let tip = job.title + "<br>" + location + "<br>"
}

makeMap([], false);