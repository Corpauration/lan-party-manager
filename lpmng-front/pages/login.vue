<template>
  <div class="d-flex flex-column justify-center align-center">
    <img src="~/assets/logo.png" style="width: 10vw; margin-bottom: 40px">
    <v-card
      class="pa-2 rounded-lg"
      max-width="800"
      min-width="320"
      width="40vw"
      style="backdrop-filter: blur(30px); background-color: #1e1e1eaa"
    >
      <v-card-actions>
        <v-spacer />
        <NuxtLink to="/register">
          <v-btn
            rounded
            color="primary"
            elevation="0"
          >
            <v-icon>mdi-account-plus</v-icon>
            Créer un compte
          </v-btn>
        </NuxtLink>
      </v-card-actions>
      <v-card-title>Connexion</v-card-title>
      <v-card-subtitle>Connectez-vous afin d'accéder à internet</v-card-subtitle>
      <v-form>
        <v-container>
          <v-text-field
            v-model="username"
            :rules="[rules.required]"
            name="username"
            placeholder="Nom d'utilisateur"
            prepend-inner-icon="mdi-account"
            :disabled="loading"
            outlined
          />
          <v-text-field
            v-model="password"
            :rules="[rules.required]"
            name="password"
            placeholder="Mot de passe"
            prepend-inner-icon="mdi-lock"
            type="password"
            :disabled="loading"
            outlined
          />
          <v-btn
            depressed
            block
            large
            color="primary"
            :loading="loading"
            :disabled="loading"
            @click="login"
          >
            CONNEXION
          </v-btn>
        </v-container>
      </v-form>
    </v-card>
    <v-snackbar
      v-model="snackbar"
    >
      Impossible de vous connecter. Avez-vous bien rentré vos informations ?

      <template #action="{ attrs }">
        <v-btn
          text
          v-bind="attrs"
          @click="snackbar = false"
        >
          Fermer
        </v-btn>
      </template>
    </v-snackbar>
  </div>
</template>

<script>
export default {
  name: 'LoginPage',
  layout: 'kiosk',
  middleware: 'notconnected',
  data () {
    return {
      loading: false,
      snackbar: false,
      username: '',
      password: '',
      rules: {
        required: value => !!value || 'Requis.'
      }
    }
  },
  methods: {
    login () {
      this.loading = true
      this.$store.getters['api/login'](this.username, this.password).then((res) => {
        if (res) {
          this.loading = false
          this.$router.push('/check')
        } else {
          this.loading = false
          this.snackbar = true
          throw 'Error'
        }
      })
    }
  }
}
</script>
