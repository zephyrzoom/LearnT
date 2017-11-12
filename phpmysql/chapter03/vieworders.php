<html>
    <body>
        <?php
        /**
         * Created by PhpStorm.
         * User: zephyr
         * Date: 4/28/14
         * Time: 10:24 PM
         *
         * 使用php显示订单
         */
        // create short variable name
        $DOCUMENT_ROOT = $_SERVER['DOCUMENT_ROOT'];
        $orders = file("$DOCUMENT_ROOT/chapter02/orders/orders.txt");
        $number_of_orders = count($orders);

        if ($number_of_orders == 0) {
            echo "<p><strong>No orders pending.
              Please try again later.</strong></p>";
        }

        for ($i = 0; $i < $number_of_orders; $i++) {
            echo $orders[$i]."<br />";
        }
        ?>
    </body>
</html>
