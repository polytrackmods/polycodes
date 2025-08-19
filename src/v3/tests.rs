use super::*;

#[test]
fn full_convert() {
    let test_values = [
        (
            "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",
            "904206f94e2512e2c96d1ecd0013058bed9abfe3ec2af0b8657697c7a2522797",
        ),
        (
            "v2IAUJXYjtGIxID4pdXdn1bjrsuYAlUUjUSWeuPnbmAyECQQeSQAuAJBIBJIPkggM8SeHJvHEgcPfN29pt9xdbrBrBbJZNb3327ttjlZpVJv7HMWNLWFLWVRyiSkfUlk0k9f3fUpkkyv93fnP8fnU8fffwkffUfhffVee1avqS1o6FK9YVjqSVoGJfTZF5NoSvnjQt5tll86rfbK9ey7ta1fUQJHrsi1rQ1KyR2xpGybcZlpKU1j3G7rLFb3oyVftXpfdqpUbKHR1SqtkjoaL12yxe6l31JvpSvnaQFt4BVnaQ5UTqF1mClX6xqN1iaS5UDqOVroOHKvC1maRNpcqBVnClXyxqO1gypmULqNtvWlZEW2h68eK5Ytf4gnPs1CqWhSPWhULdsClSxyyyOuFqVSypvlSKlQv1nH0LHrmStpc0UpEUJ613rpn6oxw6FVqUTljoKJ1SyRUZSNTOiqsULLHltfoi1ri1ri1ri9joqK1qyRVlSVlcU1kaN5omSpmSOq6StucUXpUXJHVDp2QOaoUaokjKXq5yRuSJXJH0v9eeePpQNpcqBVnqRVpKUGViSpijKKVehzHsXpUJKjKTVoqUNqO1gypmUcfbRJnWUrerv93C9E9z3e3ia6vSPRfZx6lKvpybqULdseEFKvPonow2N9Y9EFyRmllZ9KfYF2afgC1lsjVI13aJf3eN9M9C9KVMGfeKlKRZU5jVVpWVOqqUqqkrarVVNoqaVQftjVFpWROqoUqokrYrVRNoiaVF10oKL1yyRZlSZlcZbtyqBlVrKrmGVmUzkjMlSmSOzWLTNITtKTNNusSWvSybJlXJbjS2ulUXKpeLqUpmKHpKlUlcqtWqaQqaVqaaUJSNROSUKJK5EbtE1gE1qE10Y5V32IqGSthtWU5SNXOiqpUbKHR1SqtkjWqptUTbpmGVTp2UOiKXq5yRUNkaD5Ius6Wv6qfareLqTk6JyxJKloaL1o6oUi6Upe0j3afpt2PtNeTl3P1u8TtLR9kUfkW3fkekeRR9T1lfgeI9Atj2Sbo10KaJtgunmTzop0dG19oRdRtTqRthWRLo50U6WlXUTp50CaFtxYoHpdUM1V0CaONlu1o4omSzpF0KaDtjeojjeW5lWop0caBti2Q7o3Kv0ijajaHthWRLo50UafaVRPdQdoTo2UMHNpcqBVnqRVpKUZKjKRpUyx6OaKNjmT3TLol0KaNth2SfB9I9AtTN9H0j0DUc92Sbo10KaJtgunmTzop0daJjaKNjmT3TLol0fW6BaHtl2QrpV0PoH1nfHUcZPQ7ot0GaNtiWSLo7p50MaKdnxkRNlmRzp7pF0SaFtm2Qbpd0D0j0PcMTUPSPQ7ot0GaNtiWSLo7p50MaKdnjpjaKNjmT3TLol0KaNth2S7oHoHpfw5ci6R6BaHtl2QrpV0SaBdPNnmRTp7cOxomSzo5090CaJtiWTbot0O6B6R6HOndUPSPQ7ot0GaNtiWSLo7p50MaKdnrpE1UaGNnunWQLpV0aaDtl2RPQPSfhr5F1j0D0OaLthWTrol0C6e0caGNluz1kjaKNjmT3TLol0KaNth2S7oHoHpfwcGi6R6BaHtl2QrpV0SaBdPNnmRTp7MnmomSzo5090CaJtiWTbot0O6B6R6HmzVUPSPQ7ot0GaNtiWSLo7p50MaKtvFq4eCdLFX2UaGNnunWQLpV0aaDtl2RPQPSfRh6cseI9Atj2Sbo10KaJtgunmTzop0dUoV7kj1d0UaGNnunWQLpV0aaDtl2RPQhWo2Ui2vUqElRlpKUVqGVnaQ5UTqF1mO54ltj2Sbo10KaJtgunmTzop0d0tqfuVdet2Pu1etbtf9WtB3qd5WtV3qdOq7op0MaOdPtgWSro10GaLtjeA9oe5R9Atj2Sbo10KaJtgunmTzop0d0tGfLq7op0MaOdPtgWSro10GaLtjeA9ojFi6BaHtl2QrpV0SaBdPNnmRTp7obdcZU3RTpZ0c6e0CaJtiWTbot0O6B6Rj7j6BaHtl2QrpV0SaBdPNnmRTp7c0YUTpZ0c6e0CaJtiWTbot0O6B6RnjIqHod0WaDtmWRLpF090caGNluzZuiaKNjmT3TLol0KaNth2S7oHoHdepR9Atj2Sbo10KaJtgunmTzop0dOLfomSzo5090CaJtiWTbot0O6B6RX7JqHod0WaDtmWRLpF090caGNluzVEjaKNjmT3TLol0KaNth2S7oHo48X2vsXNzjXdsVq73P1nBQyvTPSPQ7UKPIvRtj2Sbk3tybsUiLbDtmWRLpF09K5oWRxt7a1l1W2CapUXRxltgibjZ0c1lomRTp7obl3p0d0t2aRdHNlmR73ulO89glV6wnWRQPSPQ7ot0GaNtiWSLo7p50M6taa2LHOPZQzp7pF0SaFtm2Qbpd0D0j0Po9tLvdvaflkkJvpKfdasUjakU3rqfdaMNpIHnSdoTo2ULqJlTNo6UNqKVhKTZUJKlSOWjk6I5YkSZkSeIbtRqBjUrGpmOStfI7RjsXOye8RaNGpFakWtRaJjasUHLHjVKjVyjt1GrGMWtasa6Y1ejtHN2eyx2zHr1YsWoxa1GrlccxokQrb1fONmGZ9CK169BNmirXQlMe3H0Yai1LoMl8H0Yak1LoyK5Pox0ErXQVUyVOeyGZZRFarqKHfgGTjseCqmS5DaMNx6FUdl8H0Yak1LoGK5Pox0ErXQ5K5Pox0IrXQNVyfgGTTseCqlSeH0Yak1Lo2HrQOeBNmCr3JqLfgGTTseCqjyrjldql9BNmGJHRNW5NSpsXf1Xf3EOz6TUxyGZZRNWq71v970IaShOl6QnQtpWUTKnaQ1paUVqCVmyoSUKlcsGJ1RyxIlyIl8IbtRqBjUrGpmOStfI7RjsXOye8RaNGpFakWtRaJjasUHLHjVKjVyjt1GrGMWtasa6Y1ejtHN2eyx2zHr1YsWoxa1GrlMMKJxInEj1StsoKJ1oykaUllaUVkaFLrqlV1yqZZRVXq1tsGWWU5SN3yaaZNtsWWWLLrtl12yOxyiqjUj6Upe0jX2ELLqxKlxK5x2ujVrGrOP2eox2zHrdZs2qxaJHrdeY9Hj1HNWfyx6fOWvf4IiJGHMRf8J6znof7E9vT0rORfyJ63mo3aieWT0HNRPzE9HT0LMRLeJafNR73Etfmo9bi2vJafNR73Etfmo9bi2vJafNR73Etfmo9bi2vJafFOzfPdsGThxGfgGTh9yfhj1YKc1gPoxUce2H6qGDdlkhua1QXHcorSO0VYH6qzDdl9hmVwQzoYoZjM0sbGaWQDNDqhm91QzcboZ9N0YthGrN0YthGrN0YthGrN0YthGrN0YthGrN0YthGrN0YthGrN0YthGrN0RyDd109KzcByM7hMzBJz8XyM3nMzbKzcuyMfaZmrXm5EmZOmZmf0MztNz8izMn6SHrUqElRlpKUVqGVnaQ5UTKsnfFn56Fn56Fn56Fn56Fn56Fn56Fn56Fn56Fn56Fn56Fn56Fn56Fn565jVKViyoyUFqKVjqTNocqJ1iaTnccL5d0t0EtGxPXg7obp9rXx7Nxnf2emi3Hi9q4tg4zvvHuvnJomWvicEeJsEUVqCVmyoSUKlUoinN5PX8ZtEVqUTtsSWWJLLzyyssyWWZLrilVxyqaZRVTqBV6Q7SQfC9n9pCFUNqKVhKTZUJKlSOWfC9ZrXUfiSJqPbbE1XUDi6zqfi6Lq9R9Z7bfC9Z75RtP1ijLfnLUdqGVlqQlpMqElSJHrvI1vIHfiS5LK5vYr9F1gvoW9F10YtPqPL1PLHfsS5zK5Pbr9Z1gPrW9Z10Pr2HHN9V6M6c6C1lvSnRnTXomeC1qLUruQt6C1qLUXi6c6M6rq9RdGdOFL5zpzovqGE1Z05qpfKdGdu9ovSnRnrfxXpzoYqXoHMqvSnRnrl8rUcZnZ0eLozpYqfqjUi6M6c6CHldhjBvwRoX443Lc09FO2fFOzQUfqUfV54rKlvqkPjeVb3oOTt6rUcZnb9OXpcutx52unrucu6XUnJ1zkjzUKBlTNPeyOnuQqRFSt1xKka7jVI1TssOGbE1JU7jVYERrjVI1mHrzpwIs8j150FFXR8tSp3bHvUMPsgSpSUGVmqQVpaUdqBlTNpQNo0xqJlTNo6UNqKVhKTZUJKlC75pHrUqElRlpKUVqGVnaQ5UTKsnnIvJKvEbjEb3E1lE1vE15E7HFznMoUqElRlpKUVqGVnaQ5UTKsH9yxqJlTNo6UNqKVhKTZUJKlC75veYlSloMqMVhqS1o6UDKnitBnQtVrab9ar3vtRitdURUnI1TkjTUKnokjqDdq17U59UlXUdkaH5IqfiUj6TS9TG1FaXe0h70M9pD39Z6THuj00nOcXqpPd4OXTf0h7mN9pD3hb6THur30nOcnwpPd4ujTf0h7YO9pD3Fd6THuz60nOc32pPd4OwTf0h7KPt4T6oP1j6SXTXRfeClSloMqMVjqTNocqJ1iaTnQdoTpfyx6aqL1j6TDUeGYbMQdZg63AtpDsXOwe8BaXi6Gao2qoGQ9prorpuUP6Gb3bUXuRLebUTvxIib0LcjeC3oX4G7l3YPfN6FuRvwNahuRr2N6FuRbaUXJ1rkjrUKXpkvStfV23i6aqL1j6rNIqeou01UcrdN1l6R9pYvfepeou010V031LE1V01UXqH1nGoHMqepeou010V6zj6aqL1j6TDMKJqepeou010VGXF11UXqH134vo6RdprprMONqrpuUPqPNw4vo6T9ou01G7GVXqH13o9o6Rdprd8RUdpeoep910iP9yvVoTpO0JUbqF1kypGUdqGVlqQlpMqElSJFq47o6SPdvX6Jq9SPFrX6JH9SP3mX6pm8SPziX6JG8SPveu0TL3lesqdpnUsL9kidpnu8L905fSvHBB9LF6531fShOl6QnQtpWUTKnaQ1paUVqCVmyoSUKlUoi7dJoUqElRlpKUVqGVnaQ5UTqF1mOh6QnShW3inHpL9eOeleeO5SvXMX6925SvXRX69e0leey6Sv3YX69a7Sv3dX69C8Sv3iX69q8Sv3nX69S9SvBtX6N8dvKete4bFKlKRZUZqCVlqR1pGUO1kaRtpToO0pU4sFFPPXfeCdK1hOhaTtomUO1gqT1oqUFqMlRloUK5YdN1l6R9pboB0QbthqBDVrGqmOUtfQ7RDtXO0e8haNGqFaoWyhadHqFPqboBUfUPqLdNdlSeV2aXpf2V63uSt6K10rU7vyeorsXeKtGXpF6KtaXpP6KtpXp3KqrpuUPqPNguhGabE1N0AqP1j6SXrOHVXqH1nGQ3YPPqBUfUPqLdt2vo6S9oepB0NGfLqBUfUPqLdNdlxuRdN1l6R9pB0NGRE1AqP1j6SXbcVUdpeoepB0NGdG1AqP1j6SXbMeo6S9oepB0NOSJqBUfUPqLdtj3iqL1j6TDobohOebU3QDoepeou01Ofgo6S9oepB0NOrSUDoepeou01u6y3dFnv7qQfursfbuafLLeJ9GuufSmLQJzPok5MUy8IKZuFlMfGlMHkSHPvkwcByMfBZmzQm5RkZuFZHPfGhrXW21QL76qldt2XMHuXMvuXMXvXMffei5EeFzT8Fzd8Fzn8ljnjZceOPl6QnQtpWUzjnl73ML3vZWufmZ5ebml73c3Efu7m47ubi96fmkSinpho4ZI2Tef83Tp4zJOrIyhGUtjSphYr4e6vfkfXkSuUiqF1kaTnI6MeERnxoaL1oaJ1oaK1miYjVEFHraZVFFHjqm4JaNRb0ai2oR9WNIETyaRNpcafYvaJRr1SHiLqB1kaRtLi4q19OWU3bWRdvtF192bEemblIGSkIGSkIGSkIGSkIGSkIGSkIGSkIGSkIGSkIGSkIGSkc4N1IoJvfeuc4ZROxzfokDXrJoB01y7V03pvRXSfL0F050Z0XpfZ6LUXfsb7XofVvfMfy23rcYcbZjkqQVpaUdjbjqGVlqQ7L5fbf2fmEisd7vziXOMqM88XUxIh9zE7f6kf1vt2vcoEDfug3rSHiawh4RXVRD4TE7cPpIez8N15N9socZR53REf6e54ZHBWs16UoU5NVemKvpibsJHHBeuP1i7gLomUO1gqT1oqUFbtiIga4JwKoaUdqBlTNpiIIb4ZJ6URE0OiSodE5Q7Iai2REGtjoOaHRi0OiOph2qQUq9Zxm2nFFTfs4j6zieUfs4y6ziarf78exJpHHPmLLCOfDLbfzs8fayffKW7iY2c4pwpm4Kb9izsXpYtzOENoDPlVvpwnfh6FKEBhfsII8zigwPLCCfZRQ4sDXJI8pTkTNpWU7CVs1CP5zx4E6vWE90KiUfCXJP8OhVERfl9yhY7X46eeTx2vfUc87JxsvnEtCforkvj2QroF0caKdrZpcrnsup0caBti2Q7oH9kJG1OaDtiWQzpp0tectdr5SMlmTLoV0GaHFiNJfk3eVfk3eQfknk6PoxU4tAoI10f2hUDaSx8MDpW9QJH0kiZp8piZOX8E5OsYfWxzjdYGYjNTywTYZ4p1u4bp8zFPLey9sBlf9rQEedtfred2eMfO5Pz3v45etKeCfGEvw3N55FfeVGmf7Suf9pvfJ8tXxzbQhyeyhvdqQtvXRvfP9erqtP1PZGihlV0Cdpn24vpN97F7lhUL6FeNPzyfei3MlPZe0fJzv9TmR8H2aXqfx30iHe9Y8p3v6cY7eaqL1j6TDobohqVD1f9GaA1n6Rdprp9zZ4ftDnfY4capHOjX4ZlMecf2fMZfn82ZdSCxO28izxVMfXB1iCndqkzsFiG9Zif6zEd7zOOewPcNzkDXBMMLh4Veuf5HfPO5f3i5QXV5XjqTNocqJ16YdiUPROORpciSeJ2an4aon46qxl1iaS5UDlSU5UTqltbUNpcqhaaU5UTql9toaS5UDtGRlTNpWafFVTKnaYmCnUMCpKVjqTNocqJ16Y1maRNpcqhSJqcqJ1iarGEVLqJlTNU7jKnaSto22zjqF1kypGa1iKnaSto2axjqF1kypGFzQulZv2kypGUdqGVlCzJs0hZgGecX1RkyvjIlfOis9dEZ774XfAB1y5HC6V1vErX6xKXq5yRUNkaD5Ius6UN5IqqStq8GVFpWROiLrs1LUeeP697dqY2Chett1r0WPVb9ea7sstdcabH112Ritd0ZROyeXftZGnEig173yZaty0GlpVITrQm6am9zM7dPfweuwooOGB2xo3OG53xRNdcEXHHtuXfPet7FLJML9Oer9QHf1UojfGK0xvuFd8bfi96fykfLKubxwYflPcm5gaS5Ujif6AEuuyvd4olgaS5UjCV7YlTNpWUokrfY1iaS5UokbcsypmULqt7OsRxetG2LbYPvhWjGahqfY1iaS5UokrdsypmULafSefqkPV0rkIfSkvE5LReloGVMPgwTm7pFnlKof2hZOEUbqF1kypQf7KiA5VEZxr4OhCL7Pm8f1b543O8bOT4bovF1kypQ5X6YlTNpWUokzOWtomUOFK5yHrcqJ1iabvOqWUTKnC9UV0XURfnK6zqofYMHlPW5UTqFFK5sjVLqJlThSu0xKnaStoQJneY1iaS5UDqjPvoTLUJq4XpmQtqjtWH9Rd0ftjxGd8JRdqPBrgilXDjCbYkZ9j1P9bvRUP5XhjoeTfl0Iq4vDIR9TfdEIqn8bQQUfZxvXDFvNYJJHeWylkc4NnL5PkUv4I9ya7KbPssjbLrFL3ehcbpc1jwyefx7fUzfyhfmI92fW6wvFSv93fHe211SCf1L5nerXyP9rXye6f0kf3Lunnw7bWq3VtUxrmUxwmUx1mUx6mUxfNpiJOpiTOpidOpinOpixOpi7OpiFPpiPPpiee86h4VT6riQkvKef1riJYvKOh9qYH2rinYvKGj9q4O2riFZvKeJ7VxssXFHzeqYb2ri3ZvKGo9q44Yq4MUqPH7UvJlvew7TYIHjoh0N0AqvSuvybAdDNkGZrNRe2VqXTXRfO9N6SlylqLfG9d6K6aqrt7fiy7C6c6M6r0fM9F1gvokfnpvSnRnTXQ7fZT8P49r1W6QsqO8WteBaLtmWS3Tz8useOfu9mR3TLp10W6B6H0EfmJvzWbfxPfnl8fdxR55W7clQuSN3WK3WPXNKXtMEf09sIy2ziDbPL6r9sYu2zi0aPLeV7ZRVtnFJWzOE3mD3n7D0WaNtkunmR35dNOqZ090SaNtleAFrVTssJe2JPxejYPwI9ADpboBUfeGfeIaIdDNg6b8zN65HQ99NzG1A6GaINSeG1QfWdj6GaA1n6Rdprpro43071UXqH1nGQ34bFfQRrqboBUfUPqLdNdF9dR6grpuUPqPNgi9RdprprovTfm397u6FumuiedKepAoLdNdF9dKmjLNmsrRdXTXRfO9N6S7lXJqGsfW7v7hfijGua0f9JVKmFQxxAJf1DfkaTebHep3m88hPR3kXO8p8m8v79896hjdSqc4zSNp8hPx4ksDfEjTKd4TMO5fqDzMp4tmPcfUHX89nWokfPPqOufbzefnSe3r4fff479j71fA9J6XoTpO0JUbqF1kypG0fovr3fB6T0vQnSdoTo2ULqJlTF93lefPAGLBqK",
            "7b8d2a55f246aa03d159f8dc18c254ee89826cf2e67cfdcf55bb05ed05942fbc",
        ),
    ];
    for (code, id) in test_values {
        let result = export_to_id(code);
        assert_eq!(result, Some(id.to_string()));
    }
}

#[test]
fn track_decode() {
    let test_values = [
        (
            "v2HAJJXZvpXYyB4p9YlBGZAEoB4kMDEfPff6BSKAQefPwsefAJhwiJGagReDWNMCW1AlhRI6jNgyAUNsCRNyA2cf3fvesD2cZEAAsExCkA",
            Track {
                author: None,
                name: "Ireozar".to_string(),
                track_data: vec![
                    5, 0, 1, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 128, 0, 0, 0, 3, 0, 0, 0, 252, 255,
                    127, 0, 0, 0, 16, 0, 128, 0, 254, 255, 127, 0, 0, 0, 255, 255, 127, 0, 254,
                    255, 127, 0, 0, 0, 2, 0, 128, 1, 15, 0, 3, 0, 0, 0, 1, 0, 128, 0, 0, 0, 255,
                    255, 127, 1, 0, 0, 128, 0, 0, 0, 6, 0, 128, 1, 254, 255, 127, 5, 0, 0, 2, 0,
                    128, 1, 28, 0, 1, 0, 0, 0, 253, 255, 127, 7, 0, 0, 0, 0, 128, 1,
                ],
            },
        ),
        (
            "v2HAUV2c0lmbnB4pdPOFkDACCDrDF1b6DwfI13E8m9s6sUQXCNtrdNEhhykfjd8d5JiLUfRMnYlFQ2mVGTppjVvbgOMTsmZV9e2TjqX7gbRpjmG4U8JyNsr0sxm723vi6eUHwLgsu9DmA",
            Track {
                author: None,
                name: "Testing".to_string(),
                track_data: vec![
                    5, 0, 1, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 128, 0, 0, 0, 3, 0, 0, 0, 252, 255,
                    127, 0, 0, 0, 16, 0, 128, 0, 254, 255, 127, 0, 0, 0, 255, 255, 127, 0, 254,
                    255, 127, 0, 0, 0, 2, 0, 128, 1, 15, 0, 3, 0, 0, 0, 1, 0, 128, 0, 0, 0, 255,
                    255, 127, 1, 0, 0, 128, 0, 0, 0, 6, 0, 128, 1, 254, 255, 127, 5, 0, 0, 2, 0,
                    128, 1, 28, 0, 1, 0, 0, 0, 253, 255, 127, 7, 0, 0, 0, 0, 128, 1, 77, 0, 2, 0,
                    0, 0, 254, 255, 127, 7, 0, 0, 0, 0, 128, 1, 0, 0, 242, 255, 127, 7, 0, 0, 8, 0,
                    128, 1, 1, 0, 76, 0, 1, 0, 0, 0, 1, 0, 128, 7, 0, 0, 0, 0, 128, 1, 75, 0, 1, 0,
                    0, 0, 0, 0, 128, 7, 0, 0, 4, 0, 128, 1, 2, 0,
                ],
            },
        ),
    ];
    for (code, track) in test_values {
        let result = decode_track_code(code);
        assert_eq!(result, Some(track));
    }
}

#[test]
fn data_decode() {
    let test_values = [(
        vec![
            0, 0, 1, 0, 0, 0, 255, 255, 127, 0, 0, 0, 0, 0, 128, 0, 41, 0, 2, 0, 0, 0, 0, 0, 128,
            0, 0, 0, 255, 255, 127, 0, 2, 0, 128, 0, 0, 0, 0, 0, 128, 1, 5, 0, 1, 0, 0, 0, 254,
            255, 127, 0, 0, 0, 0, 0, 128, 1, 43, 0, 1, 0, 0, 0, 255, 255, 127, 3, 0, 0, 1, 0, 128,
            1,
        ],
        TrackInfo {
            parts: vec![
                Part {
                    id: 0,
                    amount: 1,
                    blocks: vec![
                        Block {
                            x:-1,
                            y:0,
                            z:0,
                            rotation:0,
                            cp_order:None,
                        }
                    ],
                },
                Part {
                    id: 41,
                    amount: 2,
                    blocks: vec![
                        Block {
                            x: 0,
                            y: 0,
                            z: -1,
                            rotation: 0,
                            cp_order: None,
                        },
                        Block {
                            x: 2,
                            y: 0,
                            z: 0,
                            rotation: 1,
                            cp_order: None,
                        },
                    ],
                },
                Part {
                    id: 5,
                    amount: 1,
                    blocks: vec![Block {
                        x: -2,
                        y: 0,
                        z: 0,
                        rotation: 1,
                        cp_order: None,
                    }],
                },
                Part {
                    id: 43,
                    amount: 1,
                    blocks: vec![Block {
                        x: -1,
                        y: 3,
                        z: 1,
                        rotation: 1,
                        cp_order: None,
                    }],
                },
            ],
        },
    )];
    for (data, track_data) in test_values {
        let result = decode_track_data(&data);
        assert_eq!(result, Some(track_data));
    }
}
