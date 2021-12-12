use wypozyczalnia
db.createRole({
    "role" : "admin",
    "privileges":[
        {"resource":{"db":"wypozyczalnia","collection":""},"actions":["find","insert","update","remove"]}
    ],
    "roles":[]
})
db.createUser( { user: "superUser",
                 pwd: "12345",  // Or  "<cleartext password>"
                 roles: [ { role: "admin", db: "wypozyczalnia" },
                          { role: "readAnyDatabase", db: "admin" },
                          "readWrite"] },
               { w: "majority" , wtimeout: 5000 } )

db.filmy.insertMany([ {
            "title": "Skazani na Shawshank",
            "director": "Frank Darabont ",
            "actors": ["Tim RobbinsMorgan", "FreemanBob Gunton"],
            "genre": ["drama"],
            "score": 9.3,
            "length":2,
            "short_desc": "Two imprisoned men bond over a number of years, finding solace and eventual redemption through acts of common decency."
        },
     {
            "title": "Ojciec chrzestny",
            "director": "Francis Ford Coppola",
            "actors": ["Marlon BrandoAl", "PacinoJames Caan"],
            "genre": ["drama", "kriminal"],
            "score": 9.2,
            "length":2,
            "short_desc": "The aging patriarch of an organized crime dynasty in postwar New York City transfers control of his clandestine empire to his reluctant youngest son."
        },
     {
            "title": "Mroczny Rycerz",
            "director": "Christopher Nolan",
            "actors": ["Christian BaleHeath", "LedgerAaron Eckhart"],
            "genre": ["akcja", "kriminal", "drama"],
            "score": 9.0,
            "length":2,
            "short_desc": "When the menace known as the Joker wreaks havoc and chaos on the people of Gotham, Batman must accept one of the greatest psychological and physical tests of his ability to fight injustice."
        },
     {
            "title": "Dwunastu gniewnych ludzi",
            "director": "Sidney Lumet",
            "actors": ["Henry FondaLee", "J. Cobb", "Martin Balsam"],
            "genre": ["kriminal", "drama"],
            "score": 9.0,
            "length":2,
            "short_desc": "The jury in a New York City murder trial is frustrated by a single member whose skeptical caution forces them to more carefully consider the evidence before jumping to a hasty verdict."
        },
     {
            "title": "Lista Schindlera",
            "director": "Steven Spielberg",
            "actors": ["Liam NeesonRalph", "FiennesBen Kingsley"],
            "genre": ["biografia", "drama", "historyjny"],
            "score": 8.9,
            "length":2,
            "short_desc": "In German-occupied Poland during World War II, industrialist Oskar Schindler gradually becomes concerned for his Jewish workforce after witnessing their persecution by the Nazis."
        },
     {
            "title": "Czerwona nota",
            "director": "Rawson Marshall Thurber",
            "score": 6.4,
            "length":2,
            "actors": ["Dwayne Johnson", "Gal Gadot"],
            "genre": ["akcja", "drama"],
            "short_desc": "An Interpol agent tracks the world's most wanted art thief."
        },
     {
            "title": "Władca Pierścieni: Powrót króla",
            "director": "Peter Jackson ",
            "score": 7.2,
            "length":2,
            "actors": ["Elijah WoodViggo", "MortensenIan McKellen"],
            "genre": ["akcja", "drama"],
            "short_desc": "Gandalf and Aragorn lead the World of Men against Sauron's army to draw his gaze from Frodo and Sam as they approach Mount Doom with the One Ring."
        },
     {
            "title": "Pogromcy duchów. Dziedzictwo",
            "director": "Jason Reitman",
            "score": 6.2,
            "length":2,
            "actors": ["Carrie Coon", "Paul Rudd"],
            "genre": ["fantazja", "drama"],
            "short_desc": "When a single mom and her two kids arrive in a small town, they begin to discover their connection to the original Ghostbusters and the secret legacy their grandfather left behind."

        },
     {
            "title": "Dom Gucci",
            "director": "Ridley Scott",
            "score": 7.0,
            "length":2,
            "actors": ["Lady Gaga", "Adam Driver"],
            "genre": ["akcja", "drama"],
            "short_desc": "When Patrizia Reggiani, an outsider from humble beginnings, marries into the Gucci family, her unbridled ambition begins to unravel their legacy and triggers a reckless spiral of betrayal, decadence, revenge, and ultimately...murder."
        },
     {
            "title": "tick, tick...Boom!",
            "director": "Lin-Manuel Mirandapeter jackson ",
            "score": 7.7,
            "actors": ["Andrew Garfield", "Alexandra Shipp"],
            "genre": ["akcja", "drama"],
            "short_desc": "On the cusp of his 30th birthday, a promising young theater composer navigates love, friendship and the pressures of life as an artist in New York City."
        }
])

db.klienci.insertMany([
    {
        "name":"Cheryl",
        "surname":"Robinson",
        "phonenumber":"912-299-3266",
        "register_date": "2021-12-05 22:27:33 UTC",
    },
    {
        "name":"Kara",
        "surname":"Morrison",
        "phonenumber":"618-361-3404",
        "register_date":"2021-12-05 22:28:16 UTC",
    },
    {
        "name":"Jason",
        "surname":"Spencer",
        "phonenumber":"626-270-2643",
        "register_date":"2021-12-05 22:28:54 UTC",
    },
    {
        "name":"Julianne",
        "surname":"Jay",
        "phonenumber":"708-930-1353",
        "register_date":"2021-12-05 22:29:13 UTC",
    },
    {
        "name":"Donald",
        "surname":"Arnold",
        "phonenumber":"240-267-1108",
        "register_date":"2021-12-05 22:31:07 UTC",
    },
])

db.wypozyczenia.insertMany([
    {
        "client":
        {
            "name":"Donald",
            "surname":"Arnold",
            "phonenumber":"240-267-1108",
            "register_date":"2021-12-05 22:31:07 UTC",
        },
        "begin_date":"2001-12-29 05:41:06",
        "end_date":"2004-08-21 08:14:07",
        "actual_end_date":"2023-10-20 19:24:26"
    },
    {
        "client":{
            "name":"Kara",
            "surname":"Morrison",
            "phonenumber":"618-361-3404",
            "register_date":"2021-12-05 22:28:16 UTC",
        },
        "begin_date":"2001-12-30 05:41:06",
        "end_date":"2004-08-21 08:14:07",
        "actual_end_date":"2023-10-20 19:24:26"
    },
    {
        "client":{
            "name":"Kara",
            "surname":"Morrison",
            "phonenumber":"618-361-3404",
            "register_date":"2021-12-05 22:28:16 UTC",
        },
        "begin_date":"2002-03-18 13:14:54",
        "end_date":"2004-08-21 08:14:07",
        "actual_end_date":"2023-10-20 19:24:26"
    },
    {
        "client":{
            "name":"Kara",
            "surname":"Morrison",
            "phonenumber":"618-361-3404",
            "register_date":"2021-12-05 22:28:16 UTC",
        },
        "begin_date":"2002-03-19 13:14:54",
        "end_date":"2004-08-21 08:14:07",
        "actual_end_date":"2023-10-20 19:24:26"
    },
    {
        "client":{
            "name":"Kara",
            "surname":"Morrison",
            "phonenumber":"618-361-3404",
            "register_date":"2021-12-05 22:28:16 UTC",
        },
        "begin_date":"2002-03-20 13:14:54",
        "end_date":"2004-08-21 08:14:07",
        "actual_end_date":"2023-10-20 19:24:26"
    },
])