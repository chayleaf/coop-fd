# fiscal-data

Библиотека для работы с фискальными данными. У меня нет ни ОФД, ни ККТ -
тут просто реализуются форматы ФД (TLV и json).

Конечно, шифрования тут нет, оно реализовано в т.н. фискальных
накопителях.

Не реализовано всё, что, связано с "массивами ФДн", т.к. я не знаю, как
именно это должно работать.

## Reference

- ОФД <-> ФНС
  - https://www.nalog.gov.ru/html/sites/www.new.nalog.ru/docs/kkt/1_2_13_230421.pdf
  - https://data.nalog.ru/html/sites/www.new.nalog.ru/docs/kkt/1_05_090321.pdf
- ККТ <-> ОФД
  - https://www.nalog.gov.ru/html/sites/www.new.nalog.ru/docs/kkt/1_2_210920.pdf
  - https://github.com/yandex/ofd
- ККТ <-> ФН
  - https://www.nalog.gov.ru/html/sites/www.new.nalog.ru/docs/kkt/1_2_05_090621.pdf
- Приказ ФНС России от 14.09.2020 № ЕД-7-20/662@
  - https://www.nalog.gov.ru/rn77/about_fts/docs/10020801/
  - Форматы данных -
    https://www.nalog.gov.ru/html/sites/www.new.nalog.ru/docs/about_fts/docs/pril2_ed_7_20662.docx
- Приказ ФНС России от 12.04.2023 № ЕД-7-20/239@
  - https://www.nalog.gov.ru/rn77/about_fts/docs/14135142/
  - Форматы данных (дополнение/поправка) -
    https://www.nalog.gov.ru/html/sites/www.new.nalog.ru/2023/about_fts/docs_fts/14135142.docx
- Криптография
  - https://rst.gov.ru:8443/file-service/file/load/1699451790827
