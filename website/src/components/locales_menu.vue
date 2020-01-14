<template>
  <v-menu
    max-height="calc(100% - 16px)"
    offset-y
    right
    bottom
    transition="slide-y-reverse-transition"
  >
    <template v-slot:activator="{ attrs, on }">
      <v-btn
        class="text--secondary text-capitalize mr-3"
        icon
        v-bind="attrs"
        v-on="on"
      >
        <v-img
          :src="currentLocale.flag"
          max-width="22px"
        />
      </v-btn>
    </template>

    <v-list
      dense
      nav
    >
      <v-list-item
        v-for="language in languages"
        :key="language.locale"
        @click="setLocale(language)"
      >
        <v-list-item-avatar
          size="24px"
        >
          <v-img
            :src="language.flag"
            width="24px"
          />
        </v-list-item-avatar>
        <v-list-item-title v-text="language.name" />
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { saveLang } from '@/utils/lang';

interface Language {
  name: string,
  locale: string,
  flag: string,
}

@Component
export default class LocalesMenu extends Vue {
  // props
  // data
  languages: Language[] = [
    {
      name: 'English',
      locale: 'en',
      flag: '/static/imgs/flags/uk.png',
    },
    {
      name: 'Français',
      locale: 'fr',
      flag: '/static/imgs/flags/france.png',
    },
    // {
    //   name: 'Español',
    //   locale: 'es',
    //   flag: '/static/imgs/flags/spain.png',
    // },
    // {
    //   name: 'Português',
    //   locale: 'pt',
    //   flag: '/static/imgs/flags/portugal.png',
    // },
  ];

  // computed
  get currentLocale(): Language {
    switch (this.$i18n.locale) {
      case 'fr':
        return this.languages[1];
      // case 'es':
      //   return this.languages[2];
      // case 'pt':
      //   return this.languages[2];
      default:
        return this.languages[0];
    }
  }
  // lifecycle
  // watch
  // methods
  setLocale(lang: Language) {
    this.$i18n.locale = lang.locale;
    saveLang(lang.locale);
  }
}
</script>

<style lang="scss" scoped>
</style>
