/**
 * Created by zephyr on 10/27/14.
 */
var xmlhttp = false;
xmlhttp = new XMLHttpRequest();

var showCalendar = true;

function showHideCalendar() {
    var objID = "calendar";
    if (showCalendar == true) {
        document.getElementById("opencloseimg").innerHTML = "hide";
        var serverPage = "calendar.php";
        showCalendar = false;

        var obj = document.getElementById(objID);
        xmlhttp.open("GET", serverPage);
        xmlhttp.onreadystatechange = function() {
            if (xmlhttp.readyState == 4 && xmlhttp.status == 200) {
                obj.innerHTML = xmlhttp.responseText;
            }
        }
        xmlhttp.send(null);
    } else {
        document.getElementById("opencloseimg").innerHTML = "show";
        showCalendar = true;
        document.getElementById(objID).innerHTML = "";
    }
}