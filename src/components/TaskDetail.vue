<script setup>
import { useRoute } from "vue-router";
import { ref, watch } from "vue";
import { useTaskStore } from "../store/task";

const store = useTaskStore();

const route = useRoute();
const taskData = ref();
</script>

<template>
  <div class="detail">{{ $route.params.id }}</div>
</template>

<style lang="scss" scoped>
.detail {
  left: 400px;
  height: 100px;
  background: cyan;
}
</style>