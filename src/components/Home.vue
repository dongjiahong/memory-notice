<script setup>
import { onMounted, reactive, ref, h, computed } from "vue";
import { Message, Modal } from "@arco-design/web-vue";
import { useTaskStore } from "../store/task";

const store = useTaskStore();
const detailVisible = ref(false);
const newTaskVisible = ref(false);
const form = reactive({
  task: "",
  start: Date.now(),
  duration: 1,
  tip: "",
});

// ---------------------------
const getDelayTasks = computed(() => {
  return store.getDelayTasks;
});
const getDetailTask = computed(() => {
  return store.getDetailTask;
});

// ---------------------------
const handleSubmit = () => {
  Message.info("提交了: " + form.start_date);
};
const handleAddNewTask = () => {
  newTaskVisible.value = true;
};
const handleShowDetailTask = (task) => {
  store.updateDetailTask(task);
  detailVisible.value = true;
};

const handleNewTaskBeforeOk = async () => {
  store.addTask(form);
};
const handleReviewDetail = async () => {
  store.reviewTask();
};

const handleCancelDetail = () => {
  detailVisible.value = false;
};
const handleCancelNewTask = () => {
  newTaskVisible.value = false;
};

// ---------------------------
onMounted(() => {
  store.fetchTask();
});

const taskItemStyle = {
  color: "red",
  backGround: "#888821",
  borderRadius: "6px",
  border: "none",
};
</script>

<template>
  <div class="home">
    <a-modal
      v-model:visible="detailVisible"
      :on-before-ok="handleReviewDetail"
      title="任务详情"
      ok-text="复习完成"
    >
      <div class="title">任务名：{{ getDetailTask.task }}</div>
      <div class="info">任务开始：{{ getDetailTask.last_date }}</div>
      <div class="info">任务用时：{{ getDetailTask.duration }}</div>
      <div class="info">复习次数：{{ getDetailTask.repetitions }}</div>
      <div class="info">下次复习：{{ getDetailTask.review_date }}</div>
      <div class="info">备注：{{ getDetailTask.tip }}</div>
    </a-modal>

    <a-modal
      v-model:visible="newTaskVisible"
      @cancel="handleCancelNewTask"
      :on-before-ok="handleNewTaskBeforeOk"
      title="新建任务"
      ok-text="提交任务"
    >
      <a-form :model="form" @submit="handleSubmit">
        <a-form-item field="task" label="任务">
          <a-input v-model="form.task" placeholder="输入任务名称" />
        </a-form-item>
        <a-form-item field="start_date" label="开始的日期">
          <a-date-picker v-model="form.start" />
        </a-form-item>
        <a-form-item field="duration" label="用时">
          <a-input-number
            v-model="form.duration"
            placeholder="输入需要用时多久"
          />
        </a-form-item>
        <a-form-item field="tip" label="备注">
          <a-input v-model="form.tip" placeholder="输入备注" />
        </a-form-item>
      </a-form>
    </a-modal>

    <div class="header">
      <a-button type="primary" shape="round" @click="handleAddNewTask">
        新加任务
      </a-button>
    </div>
    <div class="content">
      <a-row class="home-grid">
        <a-col :xs="{ span: 10, offset: 1 }" :lg="{ span: 7, offset: 1 }">
          <p>延期</p>
          <a-collapse accordion :bordered="false" class="task-item">
            <a-collapse-item
              v-for="(item, index) in getDelayTasks"
              :header="item.task"
              :key="index"
            >
              <template #extra>
                <a-button
                  type="primary"
                  size="mini"
                  @click.stop="handleShowDetailTask(item)"
                  >详情</a-button
                >
              </template>
              <div>截止时间：{{ item.last_date }}</div>
              <div>复习次数：{{ item.repetitions }}</div>
            </a-collapse-item>
          </a-collapse>
          <a-pagination :total="100" simple />
        </a-col>
        <a-col :xs="{ span: 10, offset: 1 }" :lg="{ span: 7, offset: 1 }">
          <p>今天</p>
          <a-collapse accordion :bordered="false" class="task-item">
            <a-collapse-item
              v-for="(item, index) in getDelayTasks"
              :header="item.task"
              :key="index"
            >
              <div>截止时间：{{ item.duration }}</div>
              <div>延期：{{ item.tip }}</div>
            </a-collapse-item>
          </a-collapse>
          <a-pagination :total="100" simple />
        </a-col>
        <a-col :xs="{ span: 10, offset: 1 }" :lg="{ span: 7, offset: 1 }">
          <p>本周</p>
          <a-collapse accordion :bordered="false" class="task-item">
            <a-collapse-item
              v-for="(item, index) in getDelayTasks"
              :header="item.task"
              :key="index"
            >
              <div>截止时间：{{ item.duration }}</div>
              <div>延期：{{ item.tip }}</div>
            </a-collapse-item>
          </a-collapse>
          <a-pagination :total="100" simple />
        </a-col>
      </a-row>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.home {
  height: 100vh;
}
.header {
  height: 150px;
  background: cyan;
}

.task-item {
  background: cyan;
  color: red;
}

.content {
  // background: greenyellow;

  .arco-col {
    overflow: scroll;
  }

  .delay-panel {
    height: 100%;
    overflow: scroll;
  }
}
</style>