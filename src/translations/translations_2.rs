#![allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]

use crate::Language;

pub fn new_version_available_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "A newer version is available!",
        Language::IT => "Una versione più recente è disponibile!",
        Language::RU => "Новая версия доступна!",
        Language::EL => "Μια νεότερη έκδοση είναι διαθέσιμη!",
        // Language::FA => "یک نسخه جدیدتر روی GitHub موجود است",
        Language::SV => "En nyare version finns tillgänglig!",
        Language::FI => "Uudempi versio saatavilla!",
        Language::DE => "Eine neue Version ist verfügbar!",
        Language::TR => "Daha yeni bir versiyon mevcut!",
        Language::ES => "Hay una nueva versión disponible!",
        Language::KO => "새로운 버전이 출시되었습니다!",
        Language::ZH => "新版本已在 Github 发布!",
        Language::UK => "Нова версія доступна!",
        Language::RO => "O versiune nouă este disponibilă!",
        Language::PL => "Nowsza wersja jest dostępna!",
        Language::FR => "Une nouvelle version est disponible!",
        Language::JA => "新しいバージョンが利用可能になりました!",
        Language::UZ => "Yangi versiya mavjud!",
        Language::PT => "Uma nova versão está disponível!",
    }
}

pub fn inspect_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Inspect",
        Language::IT => "Ispeziona",
        Language::FR => "Inspecter",
        Language::ES => "Inspeccionar",
        Language::PL => "Sprawdź",
        Language::DE => "Inspizieren",
        Language::RU => "Инспектировать",
        Language::SV => "Inspektera",
        Language::FI => "Tarkastele",
        Language::TR => "İncele",
        // Language::FA => "بازرسی",
        Language::KO => "검사",
        Language::ZH => "检查",
        Language::UK => "Інспектувати",
        Language::RO => "Inspectați",
        Language::JA => "検査",
        Language::UZ => "Tekshirish",
        Language::PT => "Inspecionar",
        _ => "Inspect",
    }
}

pub fn connection_details_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Connection details",
        Language::IT => "Dettagli della connessione",
        Language::RU => "Подробнее о соединении",
        Language::SV => "Anslutningsdetaljer",
        Language::FI => "Yhteyden tiedot",
        Language::DE => "Verbindungsdetails",
        Language::TR => "Bağlantı detayları",
        // Language::FA => "مشخصات اتصال",
        Language::ES => "Detalles de la conexión",
        Language::KO => "연결 상세",
        Language::ZH => "连接详情",
        Language::UK => "Деталі зʼєднання",
        Language::RO => "Detalii conexiune",
        Language::PL => "Szczegóły połączenia",
        Language::FR => "Détails de la connexion",
        Language::JA => "接続の詳細",
        Language::UZ => "Ulanish tafsilotlari",
        Language::PT => "Detalhes da conexão",
        _ => "Connection details",
    }
}

pub fn dropped_packets_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Dropped packets",
        Language::IT => "Pacchetti mancati",
        Language::RU => "Потеряно пакетов",
        Language::SV => "Tappade paket",
        Language::FI => "Pudotetut paketit",
        Language::DE => "Verlorene Pakete",
        Language::TR => "Düşen paketler",
        // Language::FA => "بسته های رها شده",
        Language::ES => "Paquetes perdidos",
        Language::KO => "손실 패킷",
        Language::ZH => "丢包计数",
        Language::UK => "Пропущені пакети",
        Language::RO => "Pachete pierdute",
        Language::PL => "Utracone pakiety",
        Language::FR => "Packets perdus",
        Language::JA => "ドロップしたパケット",
        Language::UZ => "Yig'ilgan paketlar",
        Language::PT => "Pacotes perdidos",
        _ => "Dropped packets",
    }
}

pub fn data_representation_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Data representation",
        Language::IT => "Rappresentazione dei dati",
        Language::RU => "Показывать в виде", // there is selector below: "байтов" or "пакетов"
        Language::SV => "Datarepresentation",
        Language::FI => "Tietojen esitys",
        Language::DE => "Daten Darstellung",
        Language::TR => "Veri gösterimi",
        // Language::FA => "بازنمایی داده ها", // TODO: or نمایندگی داده ها depending on context
        Language::ES => "Representación de los datos",
        Language::KO => "데이터 단위",
        Language::ZH => "图表数据",
        Language::UK => "Представлення даних",
        Language::RO => "Reprezentarea datelor",
        Language::PL => "Reprezentacja danych",
        Language::FR => "Représentation de données",
        Language::JA => "データ表示",
        Language::UZ => "Ma'lumotlarni taqdim etish",
        Language::PT => "Representação dos dados",
        _ => "Data representation",
    }
}

pub fn host_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Network host",
        Language::IT => "Host di rete",
        Language::RU => "Сетевой хост",
        Language::SV => "Nätverksvärd",
        Language::FI => "Verkkoisäntä",
        Language::DE => "Netzwerk-Host",
        Language::TR => "Ağ sunucusu",
        // Language::FA => "میزبان شبکه",
        Language::ES => "Host de red",
        Language::KO => "네트워크 호스트",
        Language::ZH => "主机",
        Language::UK => "Мережевий хост",
        Language::RO => "Host rețea",
        Language::PL => "Host sieciowy",
        Language::FR => "Host réseaux",
        Language::JA => "ネットワーク ホスト",
        Language::UZ => "Tarmoq serveri",
        Language::PT => "Host da rede",
        _ => "Network host",
    }
}

pub fn only_top_30_items_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Only the top 30 items are displayed here",
        Language::IT => "Solo i 30 maggiori elementi sono mostrati qui",
        Language::RU => "Показываются только первые 30 элементов",
        Language::SV => "Endast de 30 främsta föremål visas här",
        Language::FI => "Vain 30 parasta kohteita näytetään tässä",
        Language::DE => "Nur die obersten 30 Elemente werden hier angezeigt",
        Language::TR => "Sadece ilk 30 öğeler burda gösterilmektedir",
        // Language::FA => "تنها ۳۰ موارد برتر در اینجا نمایش داده شده اند",
        Language::ES => "Aquí sólo se muestran los 30 primeros elementos",
        Language::KO => "상위 30개의 아이템만 노출됩니다",
        Language::ZH => "仅展示前 30 个项目",
        Language::UK => "Лише верхні 30 елементи відображаються тут",
        Language::RO => "Doar primele 30 de articole sunt afișate aici",
        Language::PL => "Tylko 30 pierwszych rzeczy jest wyświetlanych",
        Language::FR => "Seuls les 30 premiers articles sont affichés ici",
        Language::JA => "上位 30 件のアイテムのみが表示されます",
        Language::UZ => "Bu erda faqat dastlabki 30 ta buyumlar ko'rsatiladi",
        Language::PT => "Apenas os 30 melhores unid são expostos aqui",
        _ => "Only the top 30 items are displayed here",
    }
}

// pub fn sort_by_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "Sort by",
//         Language::IT => "Ordina per",
//         Language::RU => "Сортировка",
//         Language::SV => "Sortera efter",
//         Language::FI => "Järjestä",
//         Language::DE => "Sortieren nach",
//         Language::TR => "Şuna göre sırala",
//         // Language::FA => "مرتب سازی بر اساس",
//         Language::ES | Language::PT => "Ordenar por",
//         Language::KO => "정렬",
//         Language::ZH => "排序",
//         Language::UK => "Сортувати за",
//         Language::RO => "Filtrează după",
//         Language::PL => "Sortuj według",
//         Language::FR => "Trier par",
//         Language::JA => "ソート",
//         Language::UZ => "Saralash turi",
//         _ => "Sort by",
//     }
// }

pub fn local_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Local network",
        Language::IT => "Rete locale",
        Language::RU => "Локальная сеть",
        Language::SV => "Lokalt nätverk",
        Language::FI => "Paikallinen verkko",
        Language::DE => "Lokales Netzwerk",
        Language::TR => "Yerel ağ",
        // Language::FA => "شبکه محلی",
        Language::ES => "Red local",
        Language::KO => "로컬 네트워크",
        Language::ZH => "局域网",
        Language::UK => "Локальна мережа",
        Language::RO => "Rețea locală",
        Language::PL => "Sieć lokalna",
        Language::FR => "Réseau local",
        Language::JA => "ローカル ネットワーク",
        Language::UZ => "Mahalliy tarmoq",
        Language::PT => "Rede local",
        _ => "Local network",
    }
}

pub fn unknown_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Unknown location",
        Language::IT => "Localizzazione sconosciuta",
        Language::RU => "Неизвестный регион",
        Language::SV => "Okänd plats",
        Language::FI => "Tuntematon sijanti",
        Language::DE => "Unbekannter Ort",
        Language::TR => "Bilinmeyen yer",
        // Language::FA => "محل نامعلوم",
        Language::ES => "Ubicación desconocida",
        Language::KO => "알 수 없는 위치",
        Language::ZH => "未知",
        Language::UK => "Невідома локація",
        Language::RO => "Locație necunoscută",
        Language::PL => "Nieznana lokalizacja",
        Language::FR => "Localisation inconnue",
        Language::JA => "不明なロケーション",
        Language::UZ => "Noma'lum joylashuv",
        Language::PT => "Localização desconhecida",
        _ => "Unknown location",
    }
}

pub fn your_network_adapter_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Your network adapter",
        Language::IT => "La tua scheda di rete",
        Language::RU => "Ваш сетевой адаптер",
        Language::SV => "Din nätverksadapter",
        Language::FI => "Sinun verkkosovitin",
        Language::DE => "Dein Netzwerk-Adapter",
        Language::TR => "Ağ adaptörün",
        // Language::FA => "مبدل شبکه شما",
        Language::ES => "Su adaptador de red",
        Language::KO => "네트워크 어댑터",
        Language::ZH => "你的网络适配器",
        Language::UK => "Ваш мережевий адаптер",
        Language::RO => "Adaptorul dvs. de rețea",
        Language::PL => "Twój adapter sieciowy",
        Language::FR => "Votre carte réseau",
        Language::JA => "自身のネットワーク アダプター",
        Language::UZ => "Sizning tarmoq adapteringiz",
        Language::PT => "Seu adaptador de rede",
        _ => "Your network adapter",
    }
}

pub fn socket_address_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Socket address",
        Language::IT => "Indirizzo del socket",
        Language::RU => "Адрес сокета",
        Language::SV => "Socketadress",
        Language::FI => "Socket osoite",
        Language::DE => "Socket Adresse",
        Language::TR => "Soket adresi",
        // Language::FA => "پریز شبکه",
        Language::ES => "Dirección del socket",
        Language::KO => "소켓 어드레스",
        Language::ZH => "套接字地址",
        Language::UK => "Адреса сокета",
        Language::RO => "Adresa socket-ului",
        Language::PL => "Adres gniazda",
        Language::FR => "Adresse du socket",
        Language::JA => "ソケット アドレス",
        Language::UZ => "Soket manzili",
        Language::PT => "Endereço da socket",
        _ => "Socket address",
    }
}

pub fn mac_address_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "MAC address",
        Language::IT => "Indirizzo MAC",
        Language::RU => "MAC адрес",
        Language::SV => "MAC-adress",
        Language::FI => "MAC-osoite",
        Language::DE => "MAC Adresse",
        Language::TR => "MAC adresi",
        // Language::FA => "آدرس MAC",
        Language::ES => "Dirección MAC",
        Language::KO => "맥 어드레스",
        Language::ZH => "MAC 地址",
        Language::UK => "MAC-адреса",
        Language::RO => "Adresa MAC",
        Language::PL => "Adres MAC",
        Language::FR => "Adresse MAC",
        Language::JA => "MAC アドレス",
        Language::UZ => "MAC manzili",
        Language::PT => "Endereço MAC",
        _ => "MAC address",
    }
}

pub fn source_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Source",
        Language::IT => "Sorgente",
        Language::RU => "Источник",
        Language::SV => "Källa",
        Language::FI => "Lähde",
        Language::DE => "Quelle",
        Language::TR => "Kaynak",
        // Language::FA => "منبع",
        Language::ES => "Origen",
        Language::KO => "소스",
        Language::ZH => "源",
        Language::UK => "Джерело",
        Language::RO => "Sursă",
        Language::PL => "Źródło",
        Language::FR => "Source",
        Language::JA => "送信元",
        Language::UZ => "Manba",
        Language::PT => "Fonte",
        _ => "Source",
    }
}

pub fn destination_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::SV => "Destination",
        Language::IT => "Destinazione",
        Language::RU => "Получатель",
        Language::FI => "Määränpää",
        Language::DE => "Ziel",
        Language::TR => "Hedef",
        // Language::FA => "مقصد",
        Language::ES | Language::PT => "Destino",
        Language::KO => "목적지",
        Language::ZH => "目标",
        Language::UK => "Призначення",
        Language::RO => "Destinație",
        Language::PL => "Miejsce docelowe",
        Language::FR => "Destination",
        Language::JA => "送信先",
        Language::UZ => "Qabul qiluvchi",
        _ => "Destination",
    }
}

pub fn fqdn_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Fully qualified domain name",
        Language::IT => "Nome di dominio completo",
        Language::RU => "Полное доменное имя",
        Language::SV => "Fullständigt domännamn",
        Language::FI => "Täysin määritelty verkkotunnus",
        Language::DE => "Vollständig qualifizierter Domain Name",
        Language::TR => "Tam nitelikli alan adı",
        // Language::FA => "نام دامنه جامع الشرایط",
        Language::ES => "Nombre de dominio completo",
        Language::KO => "절대 도메인 네임",
        Language::ZH | Language::JA => "FQDN",
        Language::UK => "Повністю визначене доменне ім'я",
        Language::RO => "Nume de domeniu complet calificat",
        Language::PL => "Pełna nazwa domeny",
        Language::FR => "Nom de domaine complètement qualifié",
        Language::UZ => "To'liq domen nomi",
        Language::PT => "Nome de domínio completo",
        _ => "Fully qualified domain name",
    }
}

pub fn administrative_entity_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Autonomous System name",
        Language::IT => "Nome del sistema autonomo",
        Language::RU => "Имя автономной системы",
        Language::SV => "Administrativ enhet",
        Language::FI => "Autonomisen järjestelmän nimi",
        Language::DE => "Name des autonomen Systems",
        Language::TR => "Yönetim varlığı",
        // Language::FA => "واحد اجرایی", // TODO: or واحد اداری depending on context
        Language::ES => "Nombre del sistema autónomo",
        Language::KO => "관리 엔티티",
        Language::ZH => "ASN 信息",
        Language::UK => "Адміністративна одиниця",
        Language::RO => "Numele sistemului autonom",
        Language::PL => "Nazwa autonomicznego systemu",
        Language::FR => "Nom du système autonome",
        Language::JA => "AS 名",
        Language::UZ => "Avtonom tizim nomi",
        Language::PT => "Entidade administrativa",
        _ => "Autonomous System name",
    }
}

pub fn transmitted_data_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Transmitted data",
        Language::IT => "Dati trasmessi",
        Language::RU => "Передано данных",
        Language::SV => "Överförd data",
        Language::FI => "Lähetetty data",
        Language::DE => "Übermittelte Daten",
        Language::TR => "Aktarılan veri",
        // Language::FA => "دادهٔ منتقل شده",
        Language::ES => "Datos transmitidos",
        Language::KO => "수신된 데이터",
        Language::ZH => "数据传输",
        Language::UK => "Передані дані",
        Language::RO => "Date transmise",
        Language::PL => "Przesłane dane",
        Language::FR => "Données transmises",
        Language::JA => "転送データ",
        Language::UZ => "Uzatilgan ma'lumotlar",
        Language::PT => "Dados transmitidos",
        _ => "Transmitted data",
    }
}

pub fn country_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Country",
        Language::IT => "Paese",
        Language::RU => "Страна",
        Language::SV => "Land",
        Language::FI => "Maa",
        Language::DE => "Land",
        Language::TR => "Ülke",
        // Language::FA => "کشور",
        Language::ES | Language::PT => "País",
        Language::KO => "국가",
        Language::ZH => "国家",
        Language::UK => "Країна",
        Language::RO => "Țară",
        Language::PL => "Kraj",
        Language::FR => "Pays",
        Language::JA => "国",
        Language::UZ => "Davlat",
        _ => "Country",
    }
}

pub fn domain_name_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Domain name",
        Language::IT => "Nome di dominio",
        Language::RU => "Доменное имя",
        Language::SV => "Domännamn",
        Language::FI => "Verkkotunnus",
        Language::DE => "Domain Name",
        Language::TR => "Alan adı",
        // Language::FA => "نام دامنه",
        Language::ES => "Nombre de dominio",
        Language::KO => "도메인 네임",
        Language::ZH => "域名",
        Language::UK => "Доменне ім'я",
        Language::RO => "Nume domeniu",
        Language::PL => "Nazwa domeny",
        Language::FR => "Nom de domaine",
        Language::JA => "ドメイン名",
        Language::UZ => "Domen nomi",
        Language::PT => "Nome do domínio",
        _ => "Domain name",
    }
}

pub fn only_show_favorites_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Only show favorites",
        Language::IT => "Mostra solo i preferiti",
        Language::RU => "Показывать только избранные",
        Language::SV => "Visa endast favoriter",
        Language::FI => "Näytä vain suosikit",
        Language::DE => "Zeige nur die Favoriten",
        Language::TR => "Sadece favorileri göster",
        // Language::FA => "فقط پسندیده ها را نمایش بده",
        Language::ES => "Mostrar solo los favoritos",
        Language::KO => "즐겨찾기만 보기",
        Language::ZH => "仅显示收藏",
        Language::UK => "Показувати лише обрані",
        Language::RO => "Arată doar favorite",
        Language::PL => "Pokaż tylko ulubione",
        Language::FR => "Afficher uniquement les favoris",
        Language::JA => "お気に入りのみを表示する",
        Language::UZ => "Faqat sevimlilarni ko'rsatish",
        Language::PT => "Apenas mostrar os favoritos",
        _ => "Only show favorites",
    }
}

// pub fn search_filters_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "Search filters",
//         Language::IT => "Filtri di ricerca",
//         Language::RU => "Фильтры для поиска",
//         Language::SV => "Sökfilter",
//         Language::FI => "Hakusuodattimet",
//         Language::DE => "Filter suchen",
//         Language::TR => "Arama filtresi",
//         // Language::FA => "صافی های جستجو",
//         Language::ES => "Filtros de búsqueda",
//         Language::KO => "검색 필터",
//         Language::ZH => "搜索条件",
//         Language::UK => "Фільтри пошуку",
//         Language::RO => "Filtre de căutare",
//         Language::PL => "Filtry wyszukiwania",
//         Language::FR => "Filtres de recherche",
//         Language::JA => "検索フィルター",
//         Language::UZ => "Qidiruv filtrlari",
//         Language::PT => "Filtros de busca",
//         _ => "Search filters",
//     }
// }

pub fn no_search_results_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "No result available according to the specified search filters",
        Language::IT => "Nessun risultato disponibile secondo i filtri di ricerca specificati",
        Language::RU => "Ничего не найдено после применения выбранных фильтров",
        Language::SV => "Inga resultat tillgängliga utifrån de angivna sökfilterna",
        Language::FI => "Ei tuloksia saatavilla määritellyille hakusuodattimille",
        Language::DE => "Keine Resultate für die gewählten Filter verfügbar",
        Language::TR => "Belirtilen arama filtrelerine göre herhangi bir sonuç bulunmamaktadır",
        // Language::FA => "هیچ نتیجه ای بر اساس صافی های جستجوی تعیین شده وجود ندارد",
        Language::ES => "Los filtros de búsqueda especificados no generan ningún resultado",
        Language::KO => "해당 검색 필터로 검색된 결과가 없습니다.",
        Language::ZH => "没有符合条件的条目",
        Language::UK => "Немає результатів згідно з обраними фільтрами пошуку",
        Language::RO => "Niciun rezultat disponibil conform filtrelor de căutare specificate",
        Language::PL => "Brak wyników zgodnych z określonymi filtrami wyszukiwania",
        Language::FR => "Aucun résultat disponible selon les filtres de recherche spécifiés",
        Language::JA => "指定されたフィルター条件で表示できる結果はありません",
        Language::UZ => "Belgilangan qidiruv filtrlari bo'yicha hech qanday natija mavjud emas",
        Language::PT => "Nenhum resultado disponível de acordo com os filtros selecionados",
        _ => "No result available according to the specified search filters",
    }
}

pub fn showing_results_translation(
    language: Language,
    start: usize,
    end: usize,
    total: usize,
) -> String {
    match language {
        Language::EN => format!("Showing {start}-{end} of {total} total results"),
        Language::IT => format!("Sono mostrati {start}-{end} di {total} risultati totali"),
        Language::RU => format!("Показываются {start}-{end} из {total} общего числа результатов"),
        Language::SV => format!("Visar {start}-{end} av {total} totala resultat"),
        Language::FI => format!("Näytetään {start}-{end} tulosta, kaikista tuloksista {total}"),
        Language::DE => format!("{start}-{end} von insgesamt {total} Resultaten werden angezeigt"),
        Language::TR => format!("{total} sonuç içinde {start}-{end}"),
        // Language::FA => format!("نمایش {start}-{end} از تمامی {total} نتیجه"),
        Language::ES => format!("Mostrando {start}-{end} de {total} resultados totales"),
        Language::KO => format!("총 {total}개의 결과 중 {start}-{end}을(를) 보여줍니다"),
        Language::ZH => format!("显示累计 {total} 条目中第 {start}-{end} 个"),
        Language::UK => format!("Показано {start}-{end} з {total} загальних результатів"),
        Language::RO => format!("Se afișează {start}-{end} din {total} rezultate"),
        Language::PL => format!("Wyświetlanie {start}-{end} z {total} wyników"),
        Language::FR => format!("Affichage de {start}-{end} de {total} résultats totaux"),
        Language::JA => format!("{total} 件中の {start}-{end} 件を表示"),
        Language::UZ => format!("Jami {total} natijadan {start}-{end} ko'rsatilyapti"),
        Language::PT => format!("Mostrando {start}-{end} de {total} resultados totais"),
        _ => format!("Showing {start}-{end} of {total} total results"),
    }
}

#[allow(dead_code)]
pub fn color_gradients_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Apply color gradients",
        Language::IT => "Applica sfumature di colore",
        Language::RU => "Применить цветовой градиент", // recheck
        Language::SV => "Applicera färggradient",
        Language::FI => "Käytä värigradientteja",
        Language::DE => "Farb-Gradienten anwenden",
        Language::TR => "Renk grandyanı uygula",
        // Language::FA => "اعمال گرادیان های رنگ",
        Language::ES => "Aplicar gradientes de color",
        Language::KO => "그라디언트 색상 적용",
        Language::ZH => "应用渐变色",
        Language::UK => "Застосувати кольорові градієнти",
        Language::RO => "Aplicați gradient de culoare",
        Language::PL => "Zastosuj gradient kolorów",
        Language::FR => "Appliquer des gradients de couleur",
        Language::JA => "グラデーションを適用する",
        Language::UZ => "Rang gradientlarini qo'llang",
        Language::PT => "Aplicar gradientes de cor",
        _ => "Apply color gradients",
    }
}
