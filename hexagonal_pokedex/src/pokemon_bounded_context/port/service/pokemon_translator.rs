use crate::pokemon_bounded_context::domain::Pokemon;
use crate::pokemon_bounded_context::port::out::{
    CacheRetrieval, CacheUpdater, ShakespeareTranslator, YodaTranslator,
};

pub struct PokemonTranslator<S, Y, R, U>
where
    S: ShakespeareTranslator,
    Y: YodaTranslator,
    R: CacheRetrieval,
    U: CacheUpdater,
{
    shakespeare_translator: S,
    yoda_translator: Y,
    cache_retrieval: R,
    cache_updater: U,
}

impl<S, Y, G, U> PokemonTranslator<S, Y, G, U>
where
    S: ShakespeareTranslator,
    Y: YodaTranslator,
    G: CacheRetrieval,
    U: CacheUpdater,
{
    pub fn new(
        shakespeare_translator: S,
        yoda_translator: Y,
        cache_retrieval: G,
        cache_updater: U,
    ) -> Self {
        PokemonTranslator {
            shakespeare_translator,
            yoda_translator,
            cache_retrieval,
            cache_updater,
        }
    }
    pub async fn translate(&self, pokemon: Pokemon) -> anyhow::Result<Pokemon> {
        match pokemon.description() {
            None => Ok(pokemon),
            Some(description) => {
                let d = self
                    .translate_description_and_update_cache(
                        pokemon.name().to_string(),
                        pokemon.is_cave_or_legendary(),
                        description,
                    )
                    .await?;
                Ok(pokemon.with_description(d))
            }
        }
    }

    async fn translate_description_and_update_cache(
        &self,
        pokemon_name: String,
        is_cave_or_legendary: bool,
        description: &str,
    ) -> anyhow::Result<String> {
        match self.cache_retrieval.get(&pokemon_name).await {
            Ok(Some(translated_description)) => Ok(translated_description),
            _ => {
                let translation = self
                    .translate_without_cache(is_cave_or_legendary, description)
                    .await?;
                self.cache_updater
                    .update(&pokemon_name, translation.clone())
                    .await?;
                Ok(translation)
            }
        }
    }

    async fn translate_without_cache(
        &self,
        is_cave_or_legendary: bool,
        description: &str,
    ) -> anyhow::Result<String> {
        if is_cave_or_legendary {
            self.yoda_translator.to_yoda(description).await
        } else {
            self.shakespeare_translator
                .to_shakespeare(description)
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use crate::pokemon_bounded_context::domain::Pokemon;
    use crate::pokemon_bounded_context::port::out::MockCacheRetrieval;
    use crate::pokemon_bounded_context::port::out::MockCacheUpdater;
    use crate::pokemon_bounded_context::port::out::MockShakespeareTranslator;
    use crate::pokemon_bounded_context::port::out::MockYodaTranslator;
    use crate::pokemon_bounded_context::port::service::pokemon_translator::PokemonTranslator;

    const POKEMON_DESCRIPTION: &str = "pokemon_description";
    const TRANSLATED_DESCRIPTION: &str = "translated_translation";
    const POKEMON_NAME: &str = "pokemon_name";

    #[tokio::test]
    async fn translate_pokemon_service_translates_to_yoda_and_cache_uncached_legendary_pokemon() {
        let legendary_pokemon = Pokemon::new(
            Some(POKEMON_DESCRIPTION.to_string()),
            None,
            true,
            POKEMON_NAME.to_string(),
        );

        let mut translate_to_shakespeare_port = MockShakespeareTranslator::new();
        let mut translate_to_yoda_port = MockYodaTranslator::new();
        let mut get_cached_description_port = MockCacheRetrieval::new();
        let mut update_cached_description_port = MockCacheUpdater::new();
        given_yoda_translation(
            &mut translate_to_shakespeare_port,
            &mut translate_to_yoda_port,
            POKEMON_DESCRIPTION,
            TRANSLATED_DESCRIPTION.to_string(),
        );
        given_cache_miss_and_update(
            &mut get_cached_description_port,
            &mut update_cached_description_port,
            POKEMON_NAME,
            TRANSLATED_DESCRIPTION.to_string(),
        );

        let translate_pokemon = PokemonTranslator::new(
            translate_to_shakespeare_port,
            translate_to_yoda_port,
            get_cached_description_port,
            update_cached_description_port,
        );
        let translated_pokemon = translate_pokemon
            .translate(legendary_pokemon)
            .await
            .unwrap();
        assert_eq!(
            translated_pokemon.description().as_deref(),
            Some(TRANSLATED_DESCRIPTION)
        )
    }

    #[tokio::test]
    async fn translate_pokemon_service_translates_and_cache_uncached_to_yoda_cave_pokemon() {
        let cave_pokemon = Pokemon::new(
            Some(POKEMON_DESCRIPTION.to_string()),
            Some("cave".to_string()),
            false,
            POKEMON_NAME.to_string(),
        );

        let mut translate_to_shakespeare_port = MockShakespeareTranslator::new();
        let mut translate_to_yoda_port = MockYodaTranslator::new();
        let mut get_cached_description_port = MockCacheRetrieval::new();
        let mut update_cached_description_port = MockCacheUpdater::new();
        given_yoda_translation(
            &mut translate_to_shakespeare_port,
            &mut translate_to_yoda_port,
            POKEMON_DESCRIPTION,
            TRANSLATED_DESCRIPTION.to_string(),
        );
        given_cache_miss_and_update(
            &mut get_cached_description_port,
            &mut update_cached_description_port,
            POKEMON_NAME,
            TRANSLATED_DESCRIPTION.to_string(),
        );

        let translate_pokemon = PokemonTranslator::new(
            translate_to_shakespeare_port,
            translate_to_yoda_port,
            get_cached_description_port,
            update_cached_description_port,
        );
        let translated_pokemon = translate_pokemon.translate(cave_pokemon).await.unwrap();
        assert_eq!(
            translated_pokemon.description().as_deref(),
            Some(TRANSLATED_DESCRIPTION)
        )
    }

    #[tokio::test]
    async fn translate_pokemon_service_translates_to_shakespeare_and_cache_uncached_correct_pokemon(
    ) {
        let non_cave_non_legendary_pokemon = Pokemon::new(
            Some(POKEMON_DESCRIPTION.to_string()),
            None,
            false,
            POKEMON_NAME.to_string(),
        );

        let mut translate_to_shakespeare_port = MockShakespeareTranslator::new();
        let mut translate_to_yoda_port = MockYodaTranslator::new();
        let mut get_cached_description_port = MockCacheRetrieval::new();
        let mut update_cached_description_port = MockCacheUpdater::new();
        given_shakespeare_translation(
            &mut translate_to_shakespeare_port,
            &mut translate_to_yoda_port,
            POKEMON_DESCRIPTION,
            TRANSLATED_DESCRIPTION.to_string(),
        );

        given_cache_miss_and_update(
            &mut get_cached_description_port,
            &mut update_cached_description_port,
            POKEMON_NAME,
            TRANSLATED_DESCRIPTION.to_string(),
        );

        let translate_pokemon = PokemonTranslator::new(
            translate_to_shakespeare_port,
            translate_to_yoda_port,
            get_cached_description_port,
            update_cached_description_port,
        );
        let translated_pokemon = translate_pokemon
            .translate(non_cave_non_legendary_pokemon)
            .await
            .unwrap();
        assert_eq!(
            translated_pokemon.description().as_deref(),
            Some(TRANSLATED_DESCRIPTION)
        )
    }

    #[tokio::test]
    async fn translate_pokemon_service_translates_retrieve_cached_description() {
        let cached_pokemon = Pokemon::new(
            Some(POKEMON_DESCRIPTION.to_string()),
            None,
            false,
            POKEMON_NAME.to_string(),
        );

        let mut translate_to_shakespeare_port = MockShakespeareTranslator::new();
        let mut translate_to_yoda_port = MockYodaTranslator::new();
        let mut get_cached_description_port = MockCacheRetrieval::new();
        let mut update_cached_description_port = MockCacheUpdater::new();
        given_cache_hit(
            &mut translate_to_shakespeare_port,
            &mut translate_to_yoda_port,
            &mut get_cached_description_port,
            &mut update_cached_description_port,
            POKEMON_NAME,
            TRANSLATED_DESCRIPTION.to_string(),
        );

        let translate_pokemon = PokemonTranslator::new(
            translate_to_shakespeare_port,
            translate_to_yoda_port,
            get_cached_description_port,
            update_cached_description_port,
        );
        let translated_pokemon = translate_pokemon.translate(cached_pokemon).await.unwrap();
        assert_eq!(
            translated_pokemon.description().as_deref(),
            Some(TRANSLATED_DESCRIPTION)
        )
    }

    fn given_yoda_translation(
        translate_to_shakespeare_port: &mut MockShakespeareTranslator,
        translate_to_yoda_port: &mut MockYodaTranslator,
        pokemon_description: &'static str,
        translation: String,
    ) {
        translate_to_shakespeare_port
            .expect_to_shakespeare()
            .times(0);
        translate_to_yoda_port
            .expect_to_yoda()
            .with(eq(pokemon_description))
            .times(1)
            .returning(move |_| Ok(translation.clone()));
    }

    fn given_shakespeare_translation(
        translate_to_shakespeare_port: &mut MockShakespeareTranslator,
        translate_to_yoda_port: &mut MockYodaTranslator,
        pokemon_description: &'static str,
        translation: String,
    ) {
        translate_to_shakespeare_port
            .expect_to_shakespeare()
            .with(eq(pokemon_description))
            .times(1)
            .returning(move |_| Ok(translation.clone()));

        translate_to_yoda_port.expect_to_yoda().times(0);
    }

    fn given_cache_miss_and_update(
        get_cached_description_port: &mut MockCacheRetrieval,
        update_cached_description_port: &mut MockCacheUpdater,
        pokemon_name: &'static str,
        translated_description: String,
    ) {
        get_cached_description_port
            .expect_get()
            .with(eq(pokemon_name))
            .times(1)
            .returning(|_| Ok(None));
        update_cached_description_port
            .expect_update()
            .with(eq(pokemon_name), eq(translated_description))
            .times(1)
            .returning(|_, _| Ok(()));
    }

    fn given_cache_hit(
        translate_to_shakespeare_port: &mut MockShakespeareTranslator,
        translate_to_yoda_port: &mut MockYodaTranslator,
        get_cached_description_port: &mut MockCacheRetrieval,
        update_cached_description_port: &mut MockCacheUpdater,
        pokemon_name: &'static str,
        translated_description: String,
    ) {
        translate_to_shakespeare_port
            .expect_to_shakespeare()
            .times(0);
        translate_to_yoda_port.expect_to_yoda().times(0);
        get_cached_description_port
            .expect_get()
            .with(eq(pokemon_name))
            .times(1)
            .returning(move |_| Ok(Some(translated_description.clone())));
        update_cached_description_port.expect_update().times(0);
    }
}
