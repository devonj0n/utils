# frozen_string_literal: true

while (now = Time.now)
  next_look_away = now + (20 * 60) # 20 minutes
  while now < next_look_away
    now = Time.now
    sleep 1
  end
  `osascript -e 'display notification "Take a minute to look outside" with title "Look away from screen"'`
end
