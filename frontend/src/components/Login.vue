<script setup>
import { useAuth0 } from '@auth0/auth0-vue';

const { loginWithRedirect } = useAuth0();

const onLogin = async () => {
  await loginWithRedirect()
}
</script>

<template>
  <div>
    <button @click="onLogin">Log in</button>
  </div>
</template>
