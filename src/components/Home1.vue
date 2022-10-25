<script setup>
import TaskDetail from "./TaskDetail.vue";
import TaskDigest from "./TaskDigest.vue";
</script>

<template>
  <a-layout>
    <a-layout-sider style="width: 100px">
      <router-link
        :to="{ name: 'task_digest', params: { task_type: 'delay' } }"
      >
        <div class="sider">延期</div>
      </router-link>
      <router-link
        :to="{ name: 'task_digest', params: { task_type: 'today' } }"
      >
        <div class="sider">今天</div>
      </router-link>
      <router-link :to="{ name: 'task_digest', params: { task_type: 'week' } }">
        <div class="sider">本周</div>
      </router-link>
    </a-layout-sider>
    <a-layout-sider style="width: 200px">
      <router-view></router-view>
    </a-layout-sider>
    <a-layout-content>
      <TaskDetail></TaskDetail>
    </a-layout-content>
  </a-layout>
</template>

<style lang="scss" scoped>
.home {
  height: 100vh;
}
.arco-layout-sider {
  height: 100vh;
  background: pink;
  overflow: scroll;
}
.sider {
  height: 200px;
}
</style>