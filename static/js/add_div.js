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

function addMapDiv() {

    var height = "900px";
    if (detectMob()) {
        height = "500px"
    }

    document.getElementById("map_holder").innerHTML = '<div class="row map" id="map" style="height: ' + height + ';"></div>' + 
        '<div class="button-group">' +
        'Page:' +
        '<input type="button" class="btn btn-sm btn-link" onclick="first_page()" value="<<">' +
        '<input type="button" class="btn btn-sm btn-link" onclick="previous_page()" value="<">' +
        '<span id="page_number"></span>' +
        '<input type="button" class="btn btn-sm btn-link" onclick="next_page()" value=">">' +
        '<input type="button" class="btn btn-sm btn-link" onclick="last_page()" value=">>">' +
        'of: <span id="total_pages"></span>' +
        '</div>';
}

addMapDiv();
