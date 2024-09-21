<template>
  <nav>
    <RouterLink :to="{ name: 'home' }">Home</RouterLink>
  </nav>
  <main>
    <RouterView />
  </main>
</template>
