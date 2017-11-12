<script src="function.js"></script>

<body>
    <div id="messagebox"></div>
    <form action="" method="post">
        Your Name<br/>
        <input id="yourname" style="width: 150px; height: 30px;" type="text"
               value="" onkeypress="autocomplete1(this.value, event);"/><br/>
        <div id="autocompletediv"></div>
        <div align="right"><a href="javascript:closetask()">close</a></div>
        <div id="createtask"></div>

    </form>
</div>

</body>