/**
 * Created by zephyr on 11/5/14.
 */
function createform (e) {
    theObject = document.getElementById("createtask");

    theObject.style.visibility = "visible";
    theObject.style.height = "200px";
    theObject.style.width = "200px";

    var posx = 0;
    var posy = 0;

    posx = e.clientX + document.body.scrollLeft;
    posy = e.clientY + document.body.scrollTop;

    theObject.style.left = posx + "px";
    theObject.style.top = posy + "px";

    var objID = "createtask";
    var serverPage = "theform.php";
    var obj = document.getElementById(objID);
    var xmlhttp = new XMLHttpRequest();
    xmlhttp.open("GET", serverPage);
    xmlhttp.onreadystatechange = function() {
        if (xmlhttp.readyState == 4 && xmlhttp.status == 200) {
            obj.innerHTML = xmlhttp.responseText;
        }
    }
    xmlhttp.send(null);
}

function closetask() {
    theObject = document.getElementById("createtask");

    theObject.style.visibility = "hidden";
    theObject.style.height = "0px";
    theObject.style.width = "0px";

    acObject = document.getElementById("autocompletediv");

    acObject.style.visibility = "hidden";
    acObject.style.height = "0px";
    acObject.style.width = "0px";
}

function findPosX(obj) {
    var curleft = 0;
    if (obj.offsetParent) {
        while (obj.offsetParent) {
            curleft += obj.offsetLeft;
            obj = obj.offsetParent;
        }
    } else if (obj.x) {
        curleft += obj.x;
    }
    return curleft;
}

function findPosY(obj) {
    var curtop = 0;
    if (obj.offsetParent) {
        while (obj.offsetParent) {
            curtop += obj.offsetTop;
            obj = obj.offsetParent;
        }
    } else if (obj.y) {
        curtop += obj.y;
    }
    return curtop;
}

function autocomplete1(thevalue, e) {
    theObject = document.getElementById("autocompletediv");

    theObject.style.visibility = "visible";
    theObject.style.width = "152px";

    var posx = 0;
    var posy = 0;

    posx = (findPosX(document.getElementById("yourname")) + 1);
    posy = (findPosY(document.getElementById("yourname")) + 23);

    theObject.style.left = posx + "px";
    theObject.style.top = posy + "px";

    var theextrachar = e.which;

    if (theextrachar == undefined) {
        theextrachar = e.keyCode;
    }

    var objID = "autocompletediv";

    if (theextrachar == 8) {
        if (thevalue.length == 1) {
            var serverPage = "autocomp.php";
        } else {
            var serverPage = "autocomp.php" + "?sstring=" + thevalue.substr(0, (thevalue.length - 1));
        }
    } else {
        var serverPage = "autocomp.php" + "?sstring=" + thevalue + String.fromCharCode(theextrachar);
    }

    var obj = document.getElementById(objID);
    var xmlhttp = new XMLHttpRequest();
    xmlhttp.open("GET", serverPage);
    xmlhttp.onreadystatechange = function() {
        if (xmlhttp.readyState == 4 && xmlhttp.status == 200) {
            obj.innerHTML = xmlhttp.responseText;
        }
    }
    xmlhttp.send(null);
}

function setvalue(thevalue) {
    acObject = document.getElementById("autocompletediv");

    acObject.style.visibility = "hidden";
    acObject.style.height = "0px";
    acObject.style.width = "0px";

    document.getElementById("yourname").value = thevalue;
}
