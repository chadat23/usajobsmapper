var max_lat = 49.371643;
var min_lat = 25.827089;
var max_long = -66.927119;
var min_long = -124.639440;

function makeMap(positions, continental_us, zoom_on_radius, radius, radius_center) {

    var sw = [min_lat, min_long];
    var ne = [max_lat, max_long];

    var locations = {};
    var lat = [];
    var long = [];
    if (positions.length > 0) {
        for (const position of positions) {
            for (const location of position.locations) {
                lat.push(parseFloat(location.latitude));
                long.push(parseFloat(location.longitude));
                if (location.name in locations) {
                    locations[location.name][0].push(position);
                } else {
                    locations[location.name] = [[position, ], [[lat[lat.length - 1], long[long.length - 1]], location.found]];
                }
            }
        }

        if (zoom_on_radius && radius_center[0] > 0.001 && radius > 0.001) {
            var radius_at_lat = Math.sin(radius_center[0]) * 24901.461 / (2 * Math.PI);
            var circumference_at_lat = Math.PI * radius_at_lat**2;
            var degrees_per_mile_at_lat = 360 / circumference_at_lat;
            var degrees = radius * degrees_per_mile_at_lat
            max_lat = radius_center[0] + 360 / 24859.734 * radius;
            min_lat = radius_center[0] - 360 / 24859.734 * radius;
            max_long = radius_center[1] + degrees;
            min_long = radius_center[1] - degrees;
            n = Math.min(max_lat, Math.max(...lat));
            e = Math.min(max_long, Math.max(...long));
            s = Math.max(min_lat, Math.min(...lat));
            w = Math.max(min_long, Math.min(...long));
        } else if (continental_us) {
            n = Math.min(max_lat, Math.max(...lat));
            e = Math.min(max_long, Math.max(...long));
            s = Math.max(min_lat, Math.min(...lat));
            w = Math.max(min_long, Math.min(...long));
        } else {
            n = Math.max(...lat);
            e = Math.max(...long);
            s = Math.min(...lat);
            w = Math.min(...long);
        }
        ne = [n, e];
        sw = [s, w];
    }

    var map = L.map('map').fitBounds([sw, ne]);
    
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png?{foo}', 
        {foo: 'bar', attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}).addTo(map);

    var location_labels = makeLabels(locations);

    for (const location of location_labels) {
        if (location["found"]) {
            let marker = L.marker(location["lat_long"]).addTo(map);
            marker.bindTooltip(location["tooltip"].substring(0, location["tooltip"].length - 4), { maxWidth : 350 });
            marker.bindPopup(location["popup"].substring(0, location["popup"].length - 4), { maxWidth : 350 });
        }
    }

    var lost_locations_tooltip = "";
    var lost_locations_popup = "";
    var lost_location;
    for (const location of location_labels) {
        if (!location["found"]) {
            lost_locations_tooltip += location["tooltip"];
            lost_locations_popup += location["popup"];
            lost_location = location["lat_long"];
        }
    }
    if (lost_locations_tooltip != "") {
        lost_locations_tooltip = "THESE JOBS ARE NOT CORRECTLY LOCATED" + 
            "<br>" + 
            "FOR WHATEVER REASON, THEY COULDN'T BE FOUND IN THE LIST OF LOCAITONS" +
            "<br>" +
            "<br>" +
            lost_locations_tooltip;

        let leafletIcon = L.icon({
            iconUrl: "/static/images/missing_locations.ico",
            iconSize: [32, 32],
            iconAnchor: [16, 16],
        })
        let marker = L.marker(lost_location, {icon: leafletIcon}).addTo(map);
        marker.bindTooltip(lost_locations_tooltip.substring(0, lost_locations_tooltip.length - 4), { maxWidth : 350 });
        marker.bindPopup(lost_locations_popup.substring(0, lost_locations_popup.length - 4), { maxWidth : 350 });
    }

    if (radius > 0.001 && radius_center[0] > 0.001) {
        var circle = L.circle(radius_center, {
            color: "red",
            fillColor: "#f03",
            fillOpacity: 0.0,
            radius: radius * 1609,
        }).addTo(map);
    }
    
    if (radius_center[0] > 0.001) {
        let leafletIcon = L.icon({
            iconUrl: "/static/images/star_icon.png",
            iconSize: [32, 32],
            iconAnchor: [16, 16],
        })
        let marker = L.marker(radius_center, {icon: leafletIcon}).addTo(map);
    }      
}
