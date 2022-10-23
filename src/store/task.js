import { defineStore } from 'pinia'
import { invoke } from "@tauri-apps/api";

// 你可以对 `defineStore()` 的返回值进行任意命名，但最好使用 store 的名字，同时以 `use` 开头且以 `Store` 结尾。（比如 `useUserStore`，`useCartStore`，`useProductStore`）
// 第一个参数是你的应用程序中 Store 的唯一 ID。
export const useTaskStore = defineStore('tasks', {
    state: () => {
        return {
            detailTask: {},
            delayTasks: [],
            todayTasks: [],
            weekTasks: [],
        }
    },
    getters: {
        getDetailTask() {
            return this.detailTask;
        },
        getDelayTasks()  {
            return this.delayTasks;
        },
        getTodayTasks() {
            return this.todayTasks;
        },
        getWeekTasks() {
            return this.weekTasks;
        }
    },
    actions: {
        // 任务详情
        updateDetailTask(task) {
            this.detailTask = task;
        },
        // 添加一个新的task
        async addTask(form){
            invoke("add_task", form).then((response)=> {
                console.log("add task: ", response)
                this.fetchTask()
            });
        },
        // 复习
        async reviewTask() {
            invoke("review_task", {
                id: this.detailTask.id,
                last: this.detailTask.review_date,
                repetitions: this.detailTask.repetitions,
                efactor: this.detailTask.efactor,
                quality: 5,
              }).then((response) => {
                console.log("review task: ", response)
                this.fetchTask()
              });
        },
        // 初始化task
        async fetchTask() {
            invoke("get_tasks").then((response) => {
                this.delayTasks = response.info.delay;
                this.todayTasks= response.info.today;
                this.weekTasks= response.info.week;
            })

        }
    },

})