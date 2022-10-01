<script>
    let msg = "Hello";
    setTimeout(() => {
        msg = "Hello 1111";
    }, 1000);
</script>
<template>
    <div id="app" class="hh ww">msg is: {{ msg }} hh {{ msg }}<span>world !</span></div>
</template>
<style>
    .my-class {
        background-color: #0f0;
    }

    .my-class p {
        background-color: aqua;
    }
</style>