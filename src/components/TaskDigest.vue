<script setup>
import { onMounted, computed } from "@vue/runtime-core";
import { useRoute } from "vue-router";

import { useTaskStore } from "../store/task";

const store = useTaskStore();
const route = useRoute();
const getList = computed(() => {
  return store.getTasksDigest(route.params.task_type);
});

onMounted(() => {
  store.fetchTask();
});
</script>

<template>
  <div v-for="item in getList" :key="item.id">
    <router-link :to="{ name: 'task_detail', params: { id: item.id } }">
      <div class="digest">{{ item.task }}</div>
    </router-link>
  </div>
</template>

<style lang="scss" scoped>
.digest {
  height: 50px;
  margin-top: 10px;
  background: greenyellow;
}
</style>