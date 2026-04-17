# hackari.online

Лендинг для платформы проведения хакатонов.

## Стек

- Vue 3 + TypeScript
- Vite
- GSAP + ScrollTrigger
- Lenis (smooth scroll)
- Lucide Icons

## Установка

```bash
npm install
```

## Разработка

```bash
npm run dev
```

## Сборка

```bash
npm run build
```

## Структура

```
src/
├── components/          # Секции лендинга
│   ├── HeroPage.vue    # Hero с 3D-геометрией
│   ├── HowItWorksPage.vue  # 3 шага с 3D-флипом
│   ├── FeaturesPage.vue # Bento Grid
│   ├── StatsPage.vue   # Счётчики + диаграмма
│   └── FooterPage.vue  # Footer с Easter egg
├── composables/        # Vue composables
│   ├── useMouse3D.ts   # Отслеживание мыши
│   └── useIntersectionAnimation.ts
├── styles/
│   ├── variables.scss  # CSS-переменные
│   └── main.scss       # Глобальные стили
└── types/              # TypeScript типы
```

## Особенности

- **Шрифты**: Satoshi + Clash Display (Fontshare)
- **Цвета**: Монохром + лаймовый акцент (#D4FF00)
- **3D**: CSS transforms с перспективой
- **Анимации**: GSAP ScrollTrigger
- **Smooth scroll**: Lenis
- **Easter egg**: Матрица с японскими иероглифами

## Ключевые эффекты

1. **Hero**: Низкополигональный куб реагирует на движение мыши
2. **Page turn**: Секции анимируются при скролле как страницы
3. **Kinetic typography**: Буквы появляются с 3D-rotation
4. **Bento cards**: 3D tilt-эффект при наведении
5. **Stats**: Анимированный SVG donut chart
