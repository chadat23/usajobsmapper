function detectMob() {
    const toMatch = [
        /Android/i,
        /webOS/i,
        /iPhone/i,
        /iPad/i,
        /iPod/i,
        /BlackBerry/i,
        /Windows Phone/i
    ];
    
    return toMatch.some((toMatchItem) => {
        return navigator.userAgent.match(toMatchItem);
    });
}

function makeLabels(locations) {
    var location_labels = [];

    for (const [location_name, info] of Object.entries(locations)) {
        toolTipText = "<strong>" + location_name + "</strong>" + ":<br>";
        popupText = "";
        for (const position of info[0]) {
            toolTipText += toolTip(position);
            popupText += popup(position);
        }
        location_labels.push({
            "tooltip": toolTipText,
            "popup": popupText,
            "lat_long": info[1][0],
            "found": info[1][1],
        });
    }

    return location_labels
}

function toolTip(job, location) {
    return job.title + 
        "<br>" +
        "<br>"
}

function popup(job) {
    return job.title + 
        '<br>' + 
        '<a href="' + job.url + '" target="_blank">' + job.url + '</a>' + 
        '<br>' + 
        '<br>'
}

function setHeight() {
    var height = "900px"
    if (detectMob()) {
        height = "500px"
    }

    document.getElementById("map_holder").innerHTML = '<div class="row map" id="map" style="height: ' + height + '"></div>';
}

setHeight();
makeMap(positions(), continental_us(), zoom_on_circle(), radius(), radius_center());
