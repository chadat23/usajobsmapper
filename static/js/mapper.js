var max_lat = 49.371643;
var min_lat = 25.827089;
var max_long = -66.927119;
var min_long = -124.639440;

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
                            locations[location.name] = [[position, ], [[lat[lat.length - 1], long[long.length - 1]], location.found]];
                        }
                    }
                } else {
                        lat.push(parseFloat(location.latitude));
                        long.push(parseFloat(location.longitude));
                    if (location in locations) {
                        locations[location.name][0].push(position);
                    } else {
                        locations[location.name] = [[position, ], [[lat[lat.length - 1], long[long.length - 1]], location.found]];
                    }
                }
            }
        }

        sw = [Math.min(...lat), Math.min(...long)];
        ne = [Math.max(...lat), Math.max(...long)];
    }

    var container = L.DomUtil.get('map'); 
    if(container != null){ 
        container._leaflet_id = null; 
    }

    var map = L.map('map').fitBounds([sw, ne]);
    
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png?{foo}', 
        {foo: 'bar', attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}).addTo(map);

    var location_labels = makeLabels(locations);

    for (const location of location_labels) {
        if (location["found"]) {
            let marker = L.marker(location["lat_long"]).addTo(map);
            marker.bindTooltip(location["tooltip"]);
            marker.bindPopup(location["popup"]);
        } else {
            let leafletIcon = L.icon({
                iconUrl: "/static/images/missing_locations.ico",
                iconSize: [32, 32],
                iconAnchor: [16, 16],
            })
            let marker = L.marker(location["lat_long"], {icon: leafletIcon}).addTo(map);
            marker.bindTooltip("THESE JOBS ARE NOT CORRECTLY LOCATED" + 
                               "<br>" + 
                               "FOR WHATEVER REASON, THEY COULDN'T BE FOUND IN THE LIST OF LOCAITONS" +
                               "<br>" +
                               "<br>" +
                               location["tooltip"]);
            marker.bindPopup(location["popup"]);
            marker._icon.classList.add("huechange");
        }
        // var circle = L.circle([lat, long], {
        //     color: "red",
        //     fillColor: "#f03",
        //     fillOpacity: 0.5,
        //     radius: <radius in meters></>
        // }).addTo(map);
    }
}

