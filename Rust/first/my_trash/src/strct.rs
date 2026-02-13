use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Day {
    Yesterday,
    Today,
    Tomorrow,
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Yesterday => "Вчера",
                Self::Today => "Сегодня",
                Self::Tomorrow => "Завтра",
            }
        )
    }
}

/// Представляет информацию о рейсе поезда, включая маршрут и температурные показатели.
///
/// Этот объект хранит данные о местоположении, дате рейса и массив
/// специфических температурных коэффициентов.
#[derive(Debug, Clone)]
pub struct Train {
    /// Название станции или города прибытия.
    pub location: String,
    /// День рейса (Yesterday, Today, Tomorrow).
    pub date: Day,
    /// Технические данные (температуры), смещенные на 2 бита влево.
    pub t: [i16; 5],
}

impl Train {
    /// Создает новый экземпляр `Train`.
    ///
    /// # Аргументы
    ///
    /// * `location` - Строка, описывающая местоположение.
    /// * `date` - Объект типа `Day`.
    /// * `t` - Массив из 5 температур в формате `i8`.
    ///
    /// # Технические детали
    ///
    /// Значения массива `t` конвертируются из `i8` в `i16` и подвергаются
    /// битовому сдвигу влево на 2 позиции ($x \times 2^2$).
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::{Train, Day};
    /// let train = Train::new("Vologda", Day::Today, [1, 2, 3, 4, 5]);
    /// assert_eq!(train.location, "Vologda");
    /// assert_eq!(train.t[0], 4); // 1 << 2 = 4
    /// ```
    pub fn new(location: &str, date: Day, t: [i8; 5]) -> Self {
        Self {
            location: location.to_string(),
            date,
            t: t.map(|x| (x as i16) << 2),
        }
    }

    /// Выводит приветственное сообщение с информацией о рейсе в консоль.
    ///
    /// Использует стандартный вывод и макрос `dbg!` для отладки.
    pub fn hello(&self) {
        let tuple = (&self.location, self.date.clone(), self.t);
        println!("Hello, {:?}!", tuple);
        dbg!("Время рейса: {:?}", tuple.1);
        dbg!(&self);
    }

    /// Переносит дату рейса на завтра.
    ///
    /// # Example
    ///
    /// ```
    /// # use my_crate::{Train, Day};
    /// let mut train = Train::new("Paris", Day::Today, [0; 5]);
    /// train.set_tomorrow();
    /// // Теперь train.date будет Day::Tomorrow
    /// ```
    pub fn set_tomorrow(&mut self) {
        self.date = Day::Tomorrow;
    }
}
