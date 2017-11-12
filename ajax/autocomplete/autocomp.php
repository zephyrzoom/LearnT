<?php
/**
 * Created by PhpStorm.
 * User: zephyr
 * Date: 11/5/14
 * Time: 3:53 PM
 */
$names = array("Lee Babin", "Joe Smith", "John Doe");
$foundarr = array();

if ($_GET['sstring'] != "") {
    for ($i = 0; $i < count($names); $i++) {
        if (substr_count(strtolower($names[$i]), strtolower($_GET['sstring'])) > 0) {
            $foundarr[] = $names[$i];
        }
    }
}

if (count($foundarr) > 0) {
    ?>
<div style="background: #cccccc; border-style: solid; border-width: 1px; border-color: #000000;">
    <?php
    for ($i = 0; $i < count($foundarr); $i++) {
    ?>
    <div style="padding: 4px; height: 14px;" onmouseover="this.style.background = '#eeeeee'"
         onmouseout="this.style.background = '#cccccc'"
         onclick="setvalue('<?php echo $foundarr[$i]; ?>')">
        <?php echo $foundarr[$i];?>
    </div><?php
    }
    ?>
    </div>
<?php
}
?>