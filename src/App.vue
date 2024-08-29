<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import FruitCard from "./components/FruitCard.vue";

const fruitData = ref([]);

async function getAllFruits() {
  fruitData.value = await invoke("get_fruits");
}

document.addEventListener("DOMContentLoaded", async (e) => {
  await getAllFruits();
});
</script>

<template>
  <div class="container">
    <h1>Welcome to Fruitopia!</h1>
    <p>Here you will find information on every type of fruit!</p>

    <div class="fruits">
      <FruitCard v-for="fruit in fruitData" :key="fruit.id" :fruit="fruit" />
    </div>
  </div>
</template>

<style scoped>

h1 {
  margin-bottom: 1rem;
}

h1 + p {
  color: rgb(192, 192, 192);
}

.fruits {
  margin: 25px;
  padding: 25px;
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  grid-gap: 2.5rem;
}
</style>
