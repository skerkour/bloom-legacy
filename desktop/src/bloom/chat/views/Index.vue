<template>
  <v-container fluid>
    <v-row>
      <v-col cols="3" class="scrollable">
        <v-list two-line avatar>
        <v-list-item-group v-model="selectedConversation" color="primary">
          <v-list-item
            v-for="(conversation, i) in conversations"
            :key="i"
          >
            <v-list-item-avatar>
              <v-img :src="conversation.avatar"></v-img>
            </v-list-item-avatar>
            <v-list-item-content class="text-left">
              <v-list-item-title v-html="conversation.title"></v-list-item-title>
              <v-list-item-subtitle v-html="conversation.subtitle"></v-list-item-subtitle>
            </v-list-item-content>
          </v-list-item>
        </v-list-item-group>
      </v-list>

      </v-col>


      <v-col cols="9">
        <h1>Chat</h1>
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';


@Component
export default class Index extends Vue {
  // props
  // data
  selectedConversation = 0;
  conversations = [
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/1.jpg',
      title: 'Brunch this weekend?',
      subtitle: "<span class='text--primary'>Ali Connors</span> &mdash; I'll be in your neighborhood doing errands this weekend. Do you want to hang out?",
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/2.jpg',
      title: 'Summer BBQ <span class="grey--text text--lighten-1">4</span>',
      subtitle: "<span class='text--primary'>to Alex, Scott, Jennifer</span> &mdash; Wish I could come, but I'm out of town this weekend.",
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/3.jpg',
      title: 'Oui oui',
      subtitle: "<span class='text--primary'>Sandra Adams</span> &mdash; Do you have Paris recommendations? Have you ever been?",
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/4.jpg',
      title: 'Birthday gift',
      subtitle: "<span class='text--primary'>Trevor Hansen</span> &mdash; Have any ideas about what we should get Heidi for her birthday?",
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },
    {
      avatar: 'https://cdn.vuetifyjs.com/images/lists/5.jpg',
      title: 'Recipe to try',
      subtitle: "<span class='text--primary'>Britta Holt</span> &mdash; We should eat this: Grate, Squash, Corn, and tomatillo Tacos.",
    },

  ];

  // computed
  // lifecycle
  // watch
  // methods
}

// import { Component, Prop, Vue } from 'vue-property-decorator';
// import { Subscription } from 'rxjs';
// import Native from '@/native';

// @Component
// export default class Index extends Vue {
//   // props
//   // data
//   subscription: Subscription | null = null;
//   result = '';
//   count = 0;

//   // computed
//   // lifecycle
//   async created() {
//     this.subscription = Native.notification.subscribe((event: any) => {
//       console.log(event);
//       this.count = event.data.count;
//     });
//     this.result = (await Native.call()).count;
//   }

//   beforeDestroy() {
//     if (this.subscription) {
//       this.subscription.unsubscribe();
//     }
//   }

//   // watch
//   // methods
// }
</script>


<style lang="scss" scoped>
.scrollable {
  overflow-y: auto;
  height: calc(100vh - 90px);
}
</style>
