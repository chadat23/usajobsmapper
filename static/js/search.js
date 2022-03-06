#![feature(string_remove_matches)]

var newScript = document.createElement('script');
newScript.type = 'text/javascript';
newScript.src = '/static/js/mapper.js';
document.getElementsByTagName('head')[0].appendChild(newScript);

const leaflet = window.leaflet;

const url = "/search/query";
const myForm = document.getElementById("search_form");


myForm.addEventListener('submit', function (e) {
    e.preventDefault();
    document.getElementById("page").value = "1";

    addMapDiv();
    run_query()
})

function run_query() {
    const formData = new FormData(myForm);

    fetch(url, {
        method: 'post',
        body: formData,
    }).then(function (response) {
        return response.json();
    }).then(function (results) {
        updatePageInfo(results);
        document.getElementById("map_holder").scrollIntoView();
    }).catch(function (error) {
        console.log(error);
    })
}

function updatePageInfo(pageInfo) {
    document.getElementById("page").value = pageInfo.current_page;
    document.getElementById("number-of-pages").value = pageInfo.number_of_pages;

    document.getElementById("total_search_results").textContent = pageInfo.total_search_results;
    document.getElementById("total_returned_jobs").textContent = pageInfo.positions.length;
    document.getElementById("total_returned_locations").textContent = pageInfo.total_returned_locations;

    document.getElementById("page_number").textContent = pageInfo.current_page;
    document.getElementById("total_pages").textContent = pageInfo.number_of_pages;

    makeMap(pageInfo.positions, pageInfo.continental_us, pageInfo.zoom_on_circle, pageInfo.radius, pageInfo.radius_center);
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

function toolTip(job) {

    if (job.low_grade == job.high_grade) {
        high_grade = "";
    } else {
        high_grade = " - " + job.high_grade;
    }

    return job.title + 
        "<br>" + 
        "Grade: " + job.low_grade + high_grade + 
        "<br>" + 
        "Number of locations: " + job.locations.length + 
        "<br>" + 
        "<br>"
}

function popup(job) {
    let see_all_locations = "";
    if (job.locations.length > 1) {
        see_all_locations = '<button type="button" class="btn btn-link location" name="location" value="'  + 
            job.id + '" onclick="get_locations(\'' + job.id + '\');">See All Locations</button>' + 
            '<br>';
    }

    return job.title + 
        '<br>' + 
        '<a href="' + job.url + '" target="_blank">' + job.url + '</a>' + 
        '<br>' + 
        see_all_locations +
        "<br>"
}

function get_locations(id) {
    document.getElementById("search_form").method = "post";
    document.getElementById("search_form").action = "/search/locations/" + id;
    document.getElementById("search_form").target = "_blank";
    
    document.search_form.submit();

    document.getElementById("search_form").target = "";
}

function first_page() {addMapDiv();
    if (1 < parseInt(document.getElementById("page").value)) {
        addMapDiv();
        document.getElementById("page").value = "1";
        run_query();
    }

}

function previous_page() {
    if (1 < parseInt(document.getElementById("page").value)) {
        addMapDiv();
        document.getElementById("page").value = (parseInt(document.getElementById("page").value) - 1).toString();
        run_query();
    }
}

function next_page() {
    if (parseInt(document.getElementById("page").value) < parseInt(document.getElementById("number-of-pages").value)) {
        addMapDiv();
        document.getElementById("page").value = (parseInt(document.getElementById("page").value) + 1).toString();
        run_query();
    }
}

function last_page() {
    if (parseInt(document.getElementById("page").value) < parseInt(document.getElementById("number-of-pages").value)) {
        addMapDiv();
        document.getElementById("page").value = document.getElementById("number-of-pages").value
        run_query();
    }
}

function setRadius() {
    if (document.getElementById("location_name").value == "") {
        document.getElementById("radius").value = "";
    } else if (document.getElementById("radius").value == "") {
        document.getElementById("radius").value = "25";
    }
}

makeMap([], false, false, 0, [0, 0]);