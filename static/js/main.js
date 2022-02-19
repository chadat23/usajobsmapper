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
        return response.text();
    }).then(function (text) {
        console.log("returned text", text);
    }).catch(function (error) {
        console.log(error);
    })
})

function makeMap() {
    var sw = (min_lat, min_long);
    var ne = (max_lat, max_long);

    var start_coords = ((sw[0] + ne[0]) / 2, (sw[1] + ne[1]) / 2)

    var map = L.map('mapid').setView([51.505, -0.09], 13);
    L.raster_layers.tileLayer()
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png?{foo}', {foo: 'bar', attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}).addTo(map);

    console.log('loaded');
}

makeMap();