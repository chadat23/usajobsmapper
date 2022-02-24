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
            console.log(typeof positions);
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

    var container = L.DomUtil.get('map'); 
    if(container != null){ 
        container._leaflet_id = null; 
    }

    var map = L.map('map').fitBounds([sw, ne]);
    
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png?{foo}', 
        {foo: 'bar', attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}).addTo(map);

    var location_labels = makeLabels(locations);

    for (const location of location_labels) {
        let marker = L.marker(location["lat_long"]).addTo(map);
        marker.bindTooltip(location["tooltip"]);
        marker.bindPopup(location["popup"]);
    }
}

function makeLabels(locations) {
    var location_labels = [];

    for (const [location_name, info] of Object.entries(locations)) {
        toolTipText = "";
        popupText = "";
        for (const position of info[0]) {
            toolTipText += toolTip(position, location_name);
            popupText += popup(position);
        }
        location_labels.push({
            "tooltip": toolTipText,
            "popup": popupText,
            "lat_long": info[1],
        });
    }

    return location_labels
}