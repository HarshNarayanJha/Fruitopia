<script setup>
const props = defineProps(["fruit"]);
</script>

<template>
  <div class="fruit">
    <img src="..." alt="...">
    <h2>{{ fruit.name }}</h2>
    <p>Family: {{ fruit.family }}</p>
    <p>Order: {{ fruit.order }}</p>
    <p>Genus: {{ fruit.genus }}</p>
    <br>
    <p>Nutritional Facts (100g):</p>
    <ul>
      <li>Calories: {{ fruit.nutritions.calories }}</li>
      <li>Fat: {{ fruit.nutritions.fat }}</li>
      <li>Sugar: {{ fruit.nutritions.sugar }}</li>
      <li>Carbohydrates: {{ fruit.nutritions.carbohydrates }}</li>
      <li>Protein: {{ fruit.nutritions.protein }}</li>
    </ul>
  </div>
</template>

<style scoped>
.fruit {
  display: flex;
  flex-direction: column;
  align-items: start;

  background: rgba(128, 128, 128, 0.4);
  padding: 20px;
  border-radius: 6px;

  img {
    width: 100%;
    max-height: max(25%, 250px);
    padding: 5px;
    margin: 10px 0;
  }

  h2 {
    padding-bottom: 1rem;
  }

  p {
    font-weight: 600;
  }

  ul {
    margin-left: 25px;

    li {
      text-align: start;
    }
  }
}
</style>
