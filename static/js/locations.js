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
