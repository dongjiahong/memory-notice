<script setup>
import { useRoute } from "vue-router";
import { computed } from "vue";
import { useTaskStore } from "../store/task";

const store = useTaskStore();

const route = useRoute();
const taskDetail = computed(() => {
  return store.getDetailTask;
});
</script>

<template>
  <div class="detail">{{ taskDetail }}</div>
</template>

<style lang="scss" scoped>
.detail {
  left: 400px;
  height: 100px;
  background: cyan;
}
</style>