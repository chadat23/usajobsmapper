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
            "lat_long": info[1][0],
            "found": info[1][1],
        });
    }

    return location_labels
}

function toolTip(job, location) {

    if (job.low_grade == job.high_grade) {
        high_grade = "";
    } else {
        high_grade = " - " + job.high_grade;
    }

    return job.title + 
        "<br>" + 
        location + 
        "<br>"
}

function popup(job) {
    return job.title + 
        '<br>' + 
        '<a href="' + job.url + '" target="_blank">' + job.url + '</a>' + 
        '<br>'
}

makeMap(positions(), continental_us());
