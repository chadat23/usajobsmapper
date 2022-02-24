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
    }).catch(function (error) {
        console.log(error);
    })
}

function updatePageInfo(pageInfo) {
    document.getElementById("page").textContent = pageInfo.current_page;
    document.getElementById("number-of-pages").textContent = pageInfo.number_of_pages;

    document.getElementById("total_search_results").textContent = pageInfo.total_search_results;
    document.getElementById("total_returned_jobs").textContent = pageInfo.positions.length;
    document.getElementById("total_returned_locations").textContent = pageInfo.total_returned_locations;

    document.getElementById("page_number").textContent = pageInfo.current_page;
    document.getElementById("total_pages").textContent = pageInfo.number_of_pages;

    makeMap(pageInfo.positions, pageInfo.continental_us);
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
        "<br>" + 
        "Grade: " + job.low_grade + high_grade + 
        "<br>" + 
        "Number of locations: " + job.locations.length + 
        "<br>"
}

function popup(job) {
    return job.title + 
        '<br>' + 
        '<a href="' + job.url + '" target="_blank">' + job.url + '</a>' + 
        '<br>' + 
        '<button type="button" class="btn btn-link" name="location" value="'  + 
        job.id + '" onclick="get_locations(\'' + job.id + '\');">See All Locations</button>' + 
        '<br>'
}

function get_locations(id) {
    document.getElementById("search_form").method = "post";
    document.getElementById("search_form").action = "/search/locations/" + id;
    
    document.search_form.submit();
}

function first_page() {
    if (1 < document.getElementById("page").textContent) {
        document.getElementById("page").value = "1";
        run_query();
    }

}

function previous_page() {
    if (1 < document.getElementById("page").textContent) {
        document.getElementById("page").value = (parseInt(document.getElementById("page").textContent) - 1).toString();
        run_query();
    }
}

function next_page() {
    if (parseInt(document.getElementById("page").textContent) < parseInt(document.getElementById("number-of-pages").textContent)) {
        document.getElementById("page").value = (parseInt(document.getElementById("page").textContent) + 1).toString();
        run_query();
    }
}

function last_page() {
    if (document.getElementById("page").textContent < document.getElementById("number-of-pages").textContent) {
        document.getElementById("page").value = document.getElementById("number-of-pages").textContent
        run_query();
    }
}

makeMap([], false);