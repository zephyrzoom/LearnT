<?php
/**
 * Created by PhpStorm.
 * User: zephyr
 * Date: 10/27/14
 * Time: 12:45 AM
 */

if ((!$_GET['month']) && (!$_GET['year'])) {
    $month = date('n');
    $year = date('Y');
} else {
    $month = $_GET['month'];
    $year = $_GET['year'];
}

$timestamp = mktime(0, 0, 0, $month, 1, $year);
$monthname = date("F", $timestamp);
?>

<table>
    <tr>
        <td><?php echo $monthname." ".$year ?></td>
    </tr>
    <tr>
        <td>Su</td>
        <td>M</td>
        <td>Tu</td>
        <td>W</td>
        <td>Th</td>
        <td>F</td>
        <td>Sa</td>
    </tr>
    <?php
    $monthstart = date("w", $timestamp);
    $lastday = date("d", mktime(0, 0, 0, $month + 1, 0, $year));
    $startdate = -$monthstart;

    $numrows = ceil(((date("t", mktime(0, 0, 0, $month + 1, 0, $year)) + $monthstart) / 7));

    for ($k = 1; $k <= $numrows; $k++) {
        ?><tr>
        <?php
        for ($i = 0; $i < 7; $i++) {
            $startdate++;
            if (($startdate <= 0) || ($startdate > $lastday)) {
                ?><td>&nbsp;</td><?php
            } else {
                if ($startdate == date("j") && $month == date("n") && $year == date("Y")) {
                    ?><td><?php echo date("j"); ?></td><?php
                } else {
                    ?><td><?php echo $startdate; ?></td><?php
                }
            }
        }
        ?></tr><?php

    }
    ?>

</table>