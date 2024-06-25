<template>
    <div>
        <button @click="pobierz_wpisy"></button>
        siema blog!
        {{ wpisy }}
        <input v-model="nowyBlog" type="text">
        <button @click="dodaj_wpisy">dodaj</button>

    </div>
</template>

<script>
import { dzien2_backend } from 'declarations/dzien2_backend/index';

export default{
    data(){
        return {
            wpisy: [],
            nowyBlog = ""
        }
    },
methods: {
    async dodajWpisy(){
        await dzien2_backend.dodaj_wpisy(this.nowyBlog)
    }
    async pobierzWpisy(){
        this.wpisy = await dzien2_backend.odczytaj_wpisy()
    }
},
}
</script>